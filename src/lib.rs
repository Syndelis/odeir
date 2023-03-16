#![feature(vec_into_raw_parts)]

pub mod cside;
pub mod rustside;
pub mod transformations;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_json_is_valid() {
        use rustside::model_from_string;
        const SIMPLE_JSON_STR: &str = include_str!("../tests/fixtures/simple.json");
        
        let model = model_from_string(SIMPLE_JSON_STR);

        assert!(model.is_ok(), "{}", model.unwrap_err());

        let model = model.unwrap();

    }

    #[test]
    fn test_render_edo_simple_json() {
        use transformations::edo::render_edo;

        const SIMPLE_JSON_STR: &str = include_str!("../tests/fixtures/simple.json");

        let model = rustside::model_from_string(SIMPLE_JSON_STR).unwrap();

        let edo = render_edo(model);

        println!("{}", edo);
    }

    #[test]
    fn test_render_edo_abc_json() {
        use transformations::edo::render_edo;

        const ABC_JSON_STR: &str = include_str!("../tests/fixtures/abc.json");

        let model = rustside::model_from_string(ABC_JSON_STR).unwrap();

        let edo = render_edo(model);

        println!("{}", edo);
    }

}
