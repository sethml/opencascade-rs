#!/usr/bin/env python3
import argparse
import glob
import os
import re
from collections import Counter, defaultdict
from dataclasses import dataclass

SKIPPED_RE = re.compile(r"^// SKIPPED: .*`([^`]+)`:(\d+) - `([^`]+)`")
REASON_RE = re.compile(r"^//   Reason: (.*)$")


@dataclass
class SkipEntry:
    module: str
    header: str
    line: int
    symbol: str
    reason: str


def categorize_reason(reason: str) -> str:
    lower = reason.lower()
    if "excluded by bindings.toml" in lower:
        return "Excluded by bindings.toml"
    if "ambiguous overload" in lower or "ambiguous with" in lower:
        return "Ambiguous overload"
    if "not cppdeletable" in lower:
        return "Not CppDeletable"
    if "rvalue reference" in lower:
        return "Rvalue reference"
    if "unresolved template type" in lower:
        return "Unresolved template type"
    if "unknown handle type" in lower or ("handle(" in lower and "unknown" in lower):
        return "Unknown Handle type"
    if "is unknown" in lower or "uses unknown type" in lower or "unknown type" in lower:
        return "Unknown/unresolved type"
    if "c-style array" in lower or "[]" in reason or re.search(r"\[[0-9]*\]", reason):
        return "C-style array"
    return "Other"


def parse_generated_skips(generated_glob: str) -> list[SkipEntry]:
    entries: list[SkipEntry] = []
    for path in glob.glob(generated_glob):
        module = os.path.basename(path).removesuffix(".rs")
        with open(path, "r", encoding="utf-8") as f:
            lines = f.readlines()

        for idx, line in enumerate(lines):
            m = SKIPPED_RE.match(line.rstrip("\n"))
            if not m:
                continue

            header, line_no, symbol = m.group(1), int(m.group(2)), m.group(3)
            reason = None
            for j in range(idx + 1, min(idx + 10, len(lines))):
                rm = REASON_RE.match(lines[j].rstrip("\n"))
                if rm:
                    reason = rm.group(1)
                    break
            if reason is None:
                continue

            entries.append(
                SkipEntry(
                    module=module,
                    header=header,
                    line=line_no,
                    symbol=symbol,
                    reason=reason,
                )
            )
    return entries


def extract_unknown_type(reason: str) -> str | None:
    m = re.search(r"unknown type '([^']+)'", reason)
    if m:
        return m.group(1)
    m = re.search(r"type '([^']+)' is unknown", reason)
    if m:
        return m.group(1)
    if "unknown Handle type" in reason:
        # Preserve exact reason text for handle-type unknowns when no concrete type appears.
        return "<unknown handle type>"
    return None


def format_markdown(entries: list[SkipEntry], top_unknown: int) -> str:
    total = len(entries)
    by_category = Counter(categorize_reason(e.reason) for e in entries)

    lines = []
    lines.append(f"Total skipped symbols: **{total}**")
    lines.append("")
    lines.append("| Count | % | Category |")
    lines.append("|------:|----:|----------|")
    for cat, count in by_category.most_common():
        pct = (count / total * 100.0) if total else 0.0
        lines.append(f"| {count} | {pct:.1f}% | {cat} |")

    unknown_types = Counter()
    for e in entries:
        ty = extract_unknown_type(e.reason)
        if ty and ty != "<unknown handle type>":
            unknown_types[ty] += 1

    if unknown_types:
        lines.append("")
        lines.append(f"Top {top_unknown} unknown types:")
        for ty, count in unknown_types.most_common(top_unknown):
            lines.append(f"- {count}: {ty}")

    by_module = Counter(e.module for e in entries)
    lines.append("")
    lines.append("Top modules by skipped symbols:")
    for module, count in by_module.most_common(10):
        lines.append(f"- {module}: {count}")

    return "\n".join(lines)


def format_text(entries: list[SkipEntry], top_unknown: int) -> str:
    total = len(entries)
    by_category = Counter(categorize_reason(e.reason) for e in entries)
    lines = [f"TOTAL\t{total}", "CATEGORIES"]
    for cat, count in by_category.most_common():
        pct = (count / total * 100.0) if total else 0.0
        lines.append(f"{cat}\t{count}\t{pct:.1f}")

    by_module = Counter(e.module for e in entries)
    lines.append("MODULES")
    for module, count in by_module.most_common(30):
        module_cats = Counter(categorize_reason(e.reason) for e in entries if e.module == module)
        parts = ", ".join(f"{k}:{v}" for k, v in module_cats.most_common())
        lines.append(f"{module}\t{count}\t{parts}")

    unknown_types = Counter()
    for e in entries:
        ty = extract_unknown_type(e.reason)
        if ty and ty != "<unknown handle type>":
            unknown_types[ty] += 1

    lines.append("UNKNOWN_TYPES")
    for ty, count in unknown_types.most_common(top_unknown):
        lines.append(f"{ty}\t{count}")

    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Compute skip statistics from generated binding module comments.")
    parser.add_argument(
        "--glob",
        default="crates/opencascade-sys/generated/*.rs",
        help="Glob pattern for generated Rust module files.",
    )
    parser.add_argument(
        "--format",
        choices=["text", "markdown"],
        default="text",
        help="Output format.",
    )
    parser.add_argument(
        "--top-unknown",
        type=int,
        default=10,
        help="Number of top unknown types to list.",
    )
    args = parser.parse_args()

    entries = parse_generated_skips(args.glob)
    output = format_markdown(entries, args.top_unknown) if args.format == "markdown" else format_text(entries, args.top_unknown)
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
