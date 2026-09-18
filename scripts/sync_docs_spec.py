#!/usr/bin/env python3
"""
Sync script for Spec-Driven Documentation (SDD).
Reads the Single Source of Truth (SSOT) from docs/spec/ and generates:
1. website/src/data/cli-spec.json for Next.js documentation portal.
2. Validates integrity of commands and topics.
Zero external dependencies (uses standard library only).
"""

import json
import os
import sys

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.path.dirname(script_dir)

    commands_spec_path = os.path.join(repo_root, "docs", "spec", "commands.json")
    topics_spec_path = os.path.join(repo_root, "docs", "spec", "topics.json")
    website_target_dir = os.path.join(repo_root, "website", "src", "data")
    website_target_file = os.path.join(website_target_dir, "cli-spec.json")

    print("🔄 [SDD Sync] Reading Single Source of Truth (SSOT) specifications...")

    if not os.path.exists(commands_spec_path):
        print(f"❌ Error: Missing commands spec at {commands_spec_path}", file=sys.stderr)
        sys.exit(1)

    if not os.path.exists(topics_spec_path):
        print(f"❌ Error: Missing topics spec at {topics_spec_path}", file=sys.stderr)
        sys.exit(1)

    with open(commands_spec_path, "r", encoding="utf-8") as f:
        commands_data = json.load(f)

    with open(topics_spec_path, "r", encoding="utf-8") as f:
        topics_data = json.load(f)

    commands = commands_data.get("commands", [])
    topics = topics_data.get("topics", [])

    print(f"   Found {len(commands)} CLI commands and {len(topics)} documentation topics.")

    # Validate commands integrity
    for cmd in commands:
        for required_field in ["id", "name", "category", "summary", "description"]:
            if required_field not in cmd:
                print(f"❌ Validation error: Command missing '{required_field}': {cmd}", file=sys.stderr)
                sys.exit(1)

    # Consolidated export for website
    consolidated = {
        "version": commands_data.get("version", "3.0"),
        "commands": commands,
        "topics": topics
    }

    os.makedirs(website_target_dir, exist_ok=True)
    with open(website_target_file, "w", encoding="utf-8") as f:
        json.dump(consolidated, f, indent=2, ensure_ascii=False)
        f.write("\n")

    print(f"✅ [SDD Sync] Successfully exported {website_target_file}")

if __name__ == "__main__":
    main()
