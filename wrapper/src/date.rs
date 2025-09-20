use crate::*;

#[wasm_bindgen]
pub struct Date(InnerDate);

#[wasm_bindgen]
impl Date {
    #[wasm_bindgen(constructor)]
    pub fn from_timestamp(timestamp: Timestamp) -> Date {
        Date(InnerDate::from_timestamp(timestamp.0))
    }

    #[wasm_bindgen]
    pub fn from_ymd(year: i64, month: i64, day: i64) -> Date {
        Date(InnerDate::from_ymd(year, month, day))
    }

    #[wasm_bindgen]
    pub fn from_ymd0(year: i64, month: i64, day: i64) -> Date {
        Date(InnerDate::from_ymd0(year, month, day))
    }

    #[wasm_bindgen(getter)]
    pub fn franciade0(&self) -> i64 {
        self.0.franciade0()
    }

    #[wasm_bindgen(getter)]
    pub fn franciade(&self) -> i64 {
        self.0.franciade()
    }

    #[wasm_bindgen(getter)]
    pub fn year0(&self) -> i64 {
        self.0.year0()
    }

    #[wasm_bindgen(getter)]
    pub fn year(&self) -> i64 {
        self.0.year()
    }

    pub fn num_month0(&self) -> i64 {
        self.0.num_month0()
    }

    #[wasm_bindgen(getter)]
    pub fn num_month(&self) -> i64 {
        self.0.num_month()
    }

    #[wasm_bindgen(getter)]
    pub fn month(&self) -> String {
        self.0.month().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn day0(&self) -> i64 {
        self.0.day0()
    }

    #[wasm_bindgen(getter)]
    pub fn day(&self) -> i64 {
        self.0.day()
    }

    #[wasm_bindgen(getter)]
    pub fn decade0(&self) -> i64 {
        self.0.decade0()
    }

    #[wasm_bindgen(getter)]
    pub fn decade(&self) -> i64 {
        self.0.decade()
    }

    #[wasm_bindgen(getter)]
    pub fn num_decade_day0(&self) -> i64 {
        self.0.num_decade_day0()
    }

    #[wasm_bindgen(getter)]
    pub fn num_decade_day(&self) -> i64 {
        self.0.num_decade_day()
    }

    #[wasm_bindgen(getter)]
    pub fn decade_day(&self) -> String {
        self.0.decade_day().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn timestamp(&self) -> Timestamp {
        Timestamp(self.0.timestamp())
    }
    
    #[wasm_bindgen]
    pub fn to_string_default(&self) -> String {
        self.0.to_string_default()
    }

    #[wasm_bindgen]
    pub fn to_string_traditional(&self) -> String {
        self.0.to_string_traditional()
    }
}
