// Represents a lexer that can traverse, peek, and stash characters from a string source
#[derive(Debug)]
pub struct Rlex {
    bytes: Vec<u8>,          
    position: usize,
    length: usize,
    max_position: usize,
}

impl Rlex {
    
    fn new(source: &str) -> Result<Rlex, String> {
        if source.len() == 0 {
            return Err("MALFORMED INPUT: rlex does not accept empty strings".to_owned());
        }
        let bytes = source.as_bytes().to_vec();
        let length = bytes.len();
        let rlex = Rlex{
            bytes: bytes,
            position: 0,
            length: length,
            max_position: length-1,
        };
        return Ok(rlex);
    }


    

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_empty_rlex_throws_error() {
        let rlex = Rlex::new("");
        if rlex.is_ok() {
            panic!("rlex should not accept empty strings");
        }
        assert!(rlex.is_err());
    }


}
