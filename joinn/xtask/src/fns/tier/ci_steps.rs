//! The CI workflow's steps, read from `.github/workflows/ci.yml`.

use super::CiStep;

/// Every step with a `run:` command, in file order, with its name and
/// whether an `if: runner.os == 'Linux'` keeps it to Linux.
pub(crate) fn ci_steps(yaml: &str) -> Vec<CiStep> {
    let mut steps = Vec::new();
    let mut name = String::new();
    let mut linux_only = false;
    for line in yaml.lines() {
        let item = line.trim_start();
        let item = match item.strip_prefix("- ") {
            Some(rest) => {
                name.clear();
                linux_only = false;
                rest.trim_start()
            }
            None => item,
        };
        if let Some(value) = item.strip_prefix("name:") {
            name = value.trim().to_owned();
        } else if let Some(value) = item.strip_prefix("if:") {
            linux_only = value.contains("runner.os == 'Linux'");
        } else if let Some(value) = item.strip_prefix("run:") {
            let run = value.trim();
            if !run.is_empty() {
                steps.push(CiStep {
                    name: name.clone(),
                    run: run.to_owned(),
                    linux_only,
                });
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::ci_steps;

    #[test]
    fn the_step_list_is_the_workflows_run_lines_in_order() -> Result<(), String> {
        let path = crate::fns::workspace_root()?.join("../.github/workflows/ci.yml");
        let yaml = std::fs::read_to_string(&path).map_err(|e| format!("ci.yml: {e}"))?;
        let runs: Vec<String> = yaml
            .lines()
            .filter_map(|l| l.trim_start().trim_start_matches("- ").strip_prefix("run:"))
            .map(str::trim)
            .filter(|r| !r.is_empty())
            .map(str::to_owned)
            .collect();
        let steps = ci_steps(&yaml);
        let listed: Vec<String> = steps.iter().map(|s| s.run.clone()).collect();
        assert_eq!(listed, runs);
        assert_eq!(
            listed.last().map(String::as_str),
            Some("cargo xtask gate all")
        );
        assert!(
            steps.iter().all(|s| !s.name.is_empty()),
            "every step is named"
        );
        assert_eq!(
            steps.iter().filter(|s| s.linux_only).count(),
            1,
            "lavapipe is the one Linux-only step"
        );
        Ok(())
    }
}
