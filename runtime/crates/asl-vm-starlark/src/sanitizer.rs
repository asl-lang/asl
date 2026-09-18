/// Sanitizes engine runtime errors to provide actionable ASL diagnostics
/// without leaking internal implementation details (such as Starlark or .star files).
pub fn sanitize_error(err_str: &str) -> String {
    let mut clean_msg = err_str.to_string();

    // 1. Remove references to internal temporary files and paths
    clean_msg = clean_msg.replace("asl_skill.star", "ASL Code");
    clean_msg = clean_msg.replace("Starlark execution error: ", "");
    clean_msg = clean_msg.replace("Starlark syntax error: ", "");
    clean_msg = clean_msg.replace("Starlark", "ASL");
    clean_msg = clean_msg.replace(".star", ".asl");

    // 2. Specific capability diagnostics with helpful hints
    if clean_msg.contains("has no attribute `env`") || clean_msg.contains("has no attribute 'env'") {
        return format!(
            "ASL Capability Error: Capability 'env' is not available in this context.\n\
             💡 Hint: Add 'capabilities: env: [\"MY_VAR\"]' in your skill's frontmatter to authorize environment access.\n\
             Original detail: {}",
            clean_msg.trim()
        );
    }

    if clean_msg.contains("has no attribute `http`") || clean_msg.contains("has no attribute 'http'") {
        return format!(
            "ASL Capability Error: Capability 'http' is not available in this context.\n\
             💡 Hint: Add 'capabilities: domains: [\"api.domain.com\"]' (or 'net: allow_domains: [...]') in frontmatter.\n\
             Original detail: {}",
            clean_msg.trim()
        );
    }

    if clean_msg.contains("has no attribute `write`") || clean_msg.contains("has no attribute 'write'") {
        return format!(
            "ASL Capability Error: Filesystem write capability 'fs.write' is not available.\n\
             💡 Hint: Add 'capabilities: fs: allow_write: [\"path/to/dir\"]' in frontmatter.\n\
             Original detail: {}",
            clean_msg.trim()
        );
    }

    if clean_msg.contains("has no attribute `read`") || clean_msg.contains("has no attribute 'read'") {
        return format!(
            "ASL Capability Error: Filesystem read capability 'fs.read' is not available.\n\
             💡 Hint: Add 'capabilities: fs: roots: [\"path/to/dir\"]' in frontmatter.\n\
             Original detail: {}",
            clean_msg.trim()
        );
    }

    // 3. String iteration diagnostic
    if clean_msg.contains("Operation (iter) not supported on type `string`")
        || clean_msg.contains("Operation (iter) not supported on type string")
    {
        return format!(
            "ASL Type Error: Strings cannot be iterated directly with 'for ... in string'.\n\
             💡 Hint: Use 'for ch in chars(string):' to iterate character-by-character.\n\
             Original detail: {}",
            clean_msg.trim()
        );
    }

    // 4. Fallback formatted ASL error
    format!("ASL Runtime Error: {}", clean_msg.trim())
}
