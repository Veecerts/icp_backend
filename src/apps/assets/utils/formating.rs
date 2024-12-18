pub fn format_id(id: u64) -> String {
    let num_section = if id < 10 {
        format!("#000000{}", id)
    } else if id < 100 {
        format!("#00000{}", id)
    } else if id < 1000 {
        format!("#0000{}", id)
    } else if id < 10000 {
        format!("#000{}", id)
    } else if id < 100000 {
        format!("#00{}", id)
    } else if id < 1000000 {
        format!("#0{}", id)
    } else {
        format!("#{}", id)
    };

    format!("VEC-{}", num_section)
}
