/// compiled terminfo format parsing (term(5))

use core::io::Reader;
use core::hashmap::HashMap;
use super::super::TermInfo;

static negone: u16 = 0xFFFF;

pub fn parse(file: @Reader) -> Result<~TermInfo, ~str> {
    // Check magic number
    let magic = file.read_le_u16();
    if (magic != 0x011A) {
        return Err(fmt!("invalid magic number. expected %x but found %x", 0x011A, magic as uint));
    }

    // Get sizes
    let names_size   = file.read_le_i16();
    let bools_size   = file.read_le_i16();
    let numbers_size = file.read_le_i16();
    let offsets_size = file.read_le_i16();
    let strings_size = file.read_le_i16();

    assert!(names_size > 0);
    assert!(bools_size > 0);
    debug!("bools_size = %?", bools_size);
    assert!(bools_size > 44, "unsupported binary format");
    assert!(numbers_size > 0);
    assert!(offsets_size > 0);
    assert!(strings_size > 0);

    // Extract names
    let mut names: ~[~str] = vec::with_capacity(3);
    let names_bytes = file.read_bytes(names_size as uint);

    if names_bytes.len() != names_size as uint {
        return Err(fmt!("invalid file: expected %d bytes in the names section but hit EOF",
                        names_size as int));
    }

    for str::from_bytes(names_bytes).each_split_char('|') |s| {
        names.push(s.to_owned());
    }

    let bools = file.read_bytes(bools_size as uint);
    let numbers = file.read_bytes(numbers_size as uint);
    let offsets = file.read_bytes(offsets_size as uint);
    let strings = file.read_bytes(strings_size as uint);

    if bools.len() != bools_size as uint {
        return Err(fmt!("invalid file: expected %d bytes in the bools section but hit EOF",
                        bools_size as int));
    }
    if numbers.len() != numbers_size as uint {
        return Err(fmt!("invalid file: expected %d bytes in the numbers section but hit EOF",
                        numbers_size as int));
    }
    if offsets.len() != offsets_size as uint {
        return Err(fmt!("invalid file: expected %d bytes in the offsets section but hit EOF",
                        offsets_size as int));
    }
    if strings.len() != strings_size as uint {
        return Err(fmt!("invalid file: expected %d bytes in the strings section but hit EOF",
                        strings_size as int));
    }

    let mut bools_map = HashMap::new();

    bools_map.insert(~"auto_left_margin", bools[0] as bool);
    bools_map.insert(~"auto_right_margin", bools[1] as bool);
    bools_map.insert(~"no_esc_ctlc", bools[2] as bool);
    bools_map.insert(~"ceol_standout_glitch", bools[3] as bool);
    bools_map.insert(~"eat_newline_glitch", bools[4] as bool);
    bools_map.insert(~"erase_overstrike", bools[5] as bool);
    bools_map.insert(~"generic_type", bools[6] as bool);
    bools_map.insert(~"hard_copy", bools[7] as bool);
    bools_map.insert(~"has_meta_key", bools[8] as bool);
    bools_map.insert(~"has_status_line", bools[9] as bool);
    bools_map.insert(~"insert_null_glitch", bools[10] as bool);
    bools_map.insert(~"memory_above", bools[11] as bool);
    bools_map.insert(~"memory_below", bools[12] as bool);
    bools_map.insert(~"move_insert_mode", bools[13] as bool);
    bools_map.insert(~"move_standout_mode", bools[14] as bool);
    bools_map.insert(~"over_strike", bools[15] as bool);
    bools_map.insert(~"status_line_esc_ok", bools[16] as bool);
    bools_map.insert(~"dest_tabs_magic_smso", bools[17] as bool);
    bools_map.insert(~"tilde_glitch", bools[18] as bool);
    bools_map.insert(~"transparent_underline", bools[19] as bool);
    bools_map.insert(~"xon_xoff", bools[20] as bool);
    bools_map.insert(~"needs_xon_xoff", bools[21] as bool);
    bools_map.insert(~"prtr_silent", bools[22] as bool);
    bools_map.insert(~"hard_cursor", bools[23] as bool);
    bools_map.insert(~"non_rev_rmcup", bools[24] as bool);
    bools_map.insert(~"no_pad_char", bools[25] as bool);
    bools_map.insert(~"non_dest_scroll_region", bools[26] as bool);
    bools_map.insert(~"can_change", bools[27] as bool);
    bools_map.insert(~"back_color_erase", bools[28] as bool);
    bools_map.insert(~"hue_lightness_saturation", bools[29] as bool);
    bools_map.insert(~"col_addr_glitch", bools[30] as bool);
    bools_map.insert(~"cr_cancels_micro_mode", bools[31] as bool);
    bools_map.insert(~"has_print_wheel", bools[32] as bool);
    bools_map.insert(~"row_addr_glitch", bools[33] as bool);
    bools_map.insert(~"semi_auto_right_margin", bools[34] as bool);
    bools_map.insert(~"cpi_changes_res", bools[35] as bool);
    bools_map.insert(~"lpi_changes_res", bools[36] as bool);
    bools_map.insert(~"backspaces_with_bs", bools[37] as bool);
    bools_map.insert(~"crt_no_scrolling", bools[38] as bool);
    bools_map.insert(~"no_correctly_working_cr", bools[39] as bool);
    bools_map.insert(~"gnu_has_meta_key", bools[40] as bool);
    bools_map.insert(~"linefeed_is_newline", bools[41] as bool);
    bools_map.insert(~"has_hardware_tabs", bools[42] as bool);
    bools_map.insert(~"return_does_clr_eol", bools[43] as bool);
    let numbers = HashMap::new();
    let strings = HashMap::new();

    Ok(~TermInfo {names: names, bools: bools_map, numbers: numbers, strings: strings})
}

#[cfg(test)]
mod test {
    use super::parse;
    use p = core::path::PosixPath;
    #[test]
    fn test_parse() {
        let res = parse(io::file_reader(&p("/usr/share/terminfo/r/rxvt-unicode-256color")).unwrap());
        if !res.is_ok() {
            fail!(res.get_err());
        }
        let r = res.get();
        for r.names.each |n| {
            info!("%s", *n);
        }
        for r.bools.each |k, v| {
            info!("%s = %b", *k, *v);
        }
    }
}
