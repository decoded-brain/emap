# SPDX-License-Identifier: MIT

"""Compile-only checks for the public mutable-iterator borrow contract."""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


CASES = {
    "shared_map": (
        "E0596",
        """use emap::Map;
fn main() {
    let map = Map::with_capacity_some(1, 7_u32);
    let shared = &map;
    *shared.iter_mut().next().unwrap().1 = 9;
}
""",
    ),
    "overlapping_iterators": (
        "E0499",
        """use emap::Map;
fn main() {
    let mut map = Map::with_capacity_some(1, 7_u32);
    let mut first = map.iter_mut();
    let mut second = map.iter_mut();
    *first.next().unwrap().1 = 8;
    *second.next().unwrap().1 = 9;
}
""",
    ),
    "held_shared_value": (
        "E0502",
        """use emap::Map;
fn main() {
    let mut map = Map::with_capacity_some(1, 7_u32);
    let shared = map.get(0).unwrap();
    *map.iter_mut().next().unwrap().1 = 9;
    assert_eq!(*shared, 7);
}
""",
    ),
    "read_while_mutable_value_is_live": (
        "E0502",
        """use emap::Map;
fn main() {
    let mut map = Map::with_capacity_some(1, 7_u32);
    let mut iterator = map.iter_mut();
    let value = iterator.next().unwrap().1;
    let shared = map.get(0).unwrap();
    *value = 9;
    assert_eq!(*shared, 9);
}
""",
    ),
    "value_lifetime_invariance": (
        "lifetime may not live long enough",
        """use emap::IterMut;
fn shorten<'iter, 'short: 'iter>(iterator: IterMut<'iter, &'static str>)
    -> IterMut<'iter, &'short str>
{
    iterator
}
fn main() {}
""",
    ),
    "distinct_slots": (
        None,
        """use emap::Map;
fn main() {
    let mut map = Map::with_capacity_some(3, 7_u32);
    let mut iterator = map.iter_mut();
    let first = iterator.next().unwrap().1;
    let second = iterator.next().unwrap().1;
    *first = 8;
    *second = 9;
    assert_eq!((*first, *second), (8, 9));
}
""",
    ),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", choices=CASES)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    cargo = os.environ.get("CARGO", "cargo")
    selected = [args.case] if args.case else list(CASES)

    with tempfile.TemporaryDirectory(prefix="emap-iter-mut-contract-") as temporary:
        fixture = Path(temporary)
        (fixture / "src").mkdir()
        manifest = fixture / "Cargo.toml"
        manifest.write_text(
            '[package]\nname = "emap-iter-mut-contract"\nversion = "0.0.0"\nedition = "2024"\n'
            f'[dependencies]\nemap = {{ path = {json.dumps(str(root))} }}\n',
            encoding="utf-8",
        )
        source = fixture / "src" / "main.rs"
        source.write_text("fn main() {}\n", encoding="utf-8")
        shutil.copyfile(root / "Cargo.lock", fixture / "Cargo.lock")
        lock = subprocess.run(
            [cargo, "generate-lockfile", "--offline", "--manifest-path", str(manifest)],
            capture_output=True, text=True, check=False,
        )
        assert lock.returncode == 0, lock.stderr

        for name in selected:
            expected, text = CASES[name]
            source.write_text(text, encoding="utf-8")
            result = subprocess.run(
                [cargo, "check", "--locked", "--offline", "--jobs", "2",
                 "--manifest-path", str(manifest), "--message-format=json"],
                capture_output=True, text=True, check=False,
            )
            if expected is None:
                assert result.returncode == 0, f"{name}: {result.stderr}\n{result.stdout}"
                print(f"{name}: accepted")
                continue

            assert result.returncode != 0, f"{name}: forbidden program compiled successfully"
            errors = [
                item["message"]
                for line in result.stdout.splitlines()
                if (item := json.loads(line)).get("reason") == "compiler-message"
                and item["message"]["level"] == "error"
            ]
            assert any(
                (error.get("code") or {}).get("code") == expected
                or expected in error["message"]
                for error in errors
            ), f"{name}: unexpected failure: {result.stderr}\n{result.stdout}"
            print(f"{name}: rejected ({expected})")

    print(f"{len(selected)} mutable-iterator contract cases passed")


if __name__ == "__main__":
    main()
