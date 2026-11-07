//! Process-local only: private wishes and context never become cache files.
use crate::{
    agent::Agent,
    suggestion::{self, Suggestion},
};
use anyhow::Result;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
#[derive(Default)]
pub struct Cache {
    entries: VecDeque<(String, String, Instant, Suggestion)>,
}
impl Cache {
    pub async fn get_or_generate(
        &mut self,
        agent: &dyn Agent,
        prompt: &str,
        bypass: bool,
    ) -> Result<Suggestion> {
        self.entries
            .retain(|(_, _, time, _)| time.elapsed() < Duration::from_secs(300));
        if !bypass {
            if let Some((_, _, _, s)) = self
                .entries
                .iter()
                .find(|(a, p, _, _)| a == agent.name() && p == prompt)
            {
                return Ok(s.clone());
            }
        }
        let result = suggestion::generate(agent, prompt).await?;
        if result.executable() {
            self.entries
                .retain(|(a, p, _, _)| a != agent.name() || p != prompt);
            if self.entries.len() >= 8 {
                self.entries.pop_front();
            }
            self.entries.push_back((
                agent.name().into(),
                prompt.into(),
                Instant::now(),
                result.clone(),
            ));
        }
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Mock(AtomicUsize);
    #[async_trait]
    impl Agent for Mock {
        fn name(&self) -> &str {
            "mock"
        }
        fn is_available(&self) -> bool {
            true
        }
        async fn run(&self, _: &str) -> Result<String> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(r#"{"command":"echo hello","explanation":"hello","danger":"low","alternatives":[]}"#.into())
        }
    }
    #[tokio::test]
    async fn exact_context_cache_and_regeneration_bypass() {
        let agent = Mock(AtomicUsize::new(0));
        let mut cache = Cache::default();
        cache.get_or_generate(&agent, "a", false).await.unwrap();
        cache.get_or_generate(&agent, "a", false).await.unwrap();
        assert_eq!(agent.0.load(Ordering::SeqCst), 1);
        cache.get_or_generate(&agent, "a", true).await.unwrap();
        cache.get_or_generate(&agent, "b", false).await.unwrap();
        assert_eq!(agent.0.load(Ordering::SeqCst), 3);
    }
}
