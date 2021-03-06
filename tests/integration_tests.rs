use assert_cmd::Command;

fn hexyl() -> Command {
    let mut cmd = Command::cargo_bin("hexyl").unwrap();
    cmd.current_dir("tests/examples");
    cmd
}

#[test]
fn can_print_simple_ascii_file() {
    hexyl()
        .arg("ascii")
        .arg("--color=never")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 61 62 63 64 65 66 67 68 ┊ 21 3f 25 26 2f 28 29 0a │abcdefgh┊!?%&/()_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn fails_on_non_existing_input() {
    hexyl().arg("non-existing").assert().failure();
}

#[test]
fn length_restricts_output_size() {
    hexyl()
        .arg("hello_world_elf64")
        .arg("--color=never")
        .arg("--length=32")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000000│ 7f 45 4c 46 02 01 01 00 ┊ 00 00 00 00 00 00 00 00 │•ELF•••0┊00000000│\n\
             │00000010│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•0>0•000┊0•@00000│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn prints_warning_on_empty_content() {
    hexyl()
        .arg("empty")
        .arg("--color=never")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │        │ No content to print     │                         │        │        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn prints_warning_when_skipping_past_the_end() {
    hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--skip=1000")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │        │ No content to print     │                         │        │        │\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn fail_if_length_and_bytes_options_are_used_simultaneously() {
    hexyl()
        .arg("hello_world_elf64")
        .arg("--length=32")
        .arg("--bytes=10")
        .assert()
        .failure();
}

#[test]
fn display_offset() {
    hexyl()
        .arg("ascii")
        .arg("--color=never")
        .arg("--display-offset=0xc0ffee")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00c0ffee│ 61 62 63 64 65 66 67 68 ┊ 21 3f 25 26 2f 28 29 0a │abcdefgh┊!?%&/()_│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn display_offset_and_skip() {
    hexyl()
        .arg("hello_world_elf64")
        .arg("--color=never")
        .arg("--display-offset=0x20")
        .arg("--skip=0x10")
        .arg("--length=0x10")
        .assert()
        .success()
        .stdout(
            "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n\
             │00000030│ 02 00 3e 00 01 00 00 00 ┊ 00 10 40 00 00 00 00 00 │•0>0•000┊0•@00000│\n\
             └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘\n",
        );
}

#[test]
fn fails_for_zero_or_negative_blocksize() {
    hexyl()
        .arg("ascii")
        .arg("--block-size=0")
        .assert()
        .failure();

    hexyl()
        .arg("ascii")
        .arg("--block-size=-16")
        .assert()
        .failure();
}
