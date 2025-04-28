use anyhow::Result;
use std::{fs, io::Write, path::Path};

const A: &str = include_str!("../../casbin/rbac_with_pattern_policy.csv");

pub fn create_policy_file() -> Result<()> {
    let path = Path::new("./casbin/rbac_with_pattern_policy.csv");
    if path.exists() {
        return Ok(());
    }
    // 创建目录
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap_or_default();
        }
    }
    // 写入文件
    let mut file = fs::File::create(path)?;
    file.write_all(A.as_bytes())?;
    Ok(())
}
