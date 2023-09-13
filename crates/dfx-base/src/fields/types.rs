
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FieldType {
    Boolean,
    Char,
    DateOnly,
    DateTime,
    Decimal,
    Int,
    String,
    TimeOnly,
}

impl FieldType {
    pub fn get(value: &str) -> Result<Self, String> {
        match value {
            "STRING" => Ok(Self::String),
            "CHAR" => Ok(Self::Char),
            "PRICE" => Ok(Self::Decimal),
            "INT" => Ok(Self::Int),
            "AMT" => Ok(Self::Decimal),
            "QTY" => Ok(Self::Decimal),
            "CURRENCY" => Ok(Self::String),
            "MULTIPLEVALUESTRING" => Ok(Self::String),
            "MULTIPLESTRINGVALUE" => Ok(Self::String),
            "MULTIPLECHARVALUE" => Ok(Self::String),
            "EXCHANGE" => Ok(Self::String),
            "UTCTIMESTAMP" => Ok(Self::DateTime),
            "BOOLEAN" => Ok(Self::Boolean),
            "LOCALMKTDATE" => Ok(Self::String),
            "LOCALMKTTIME" => Ok(Self::String),
            "DATA" => Ok(Self::String),
            "FLOAT" => Ok(Self::Decimal),
            "PRICEOFFSET" => Ok(Self::Decimal),
            "MONTHYEAR" => Ok(Self::String),
            "DAYOFMONTH" => Ok(Self::String),
            "UTCDATE" => Ok(Self::DateOnly),
            "UTCDATEONLY" => Ok(Self::DateOnly),
            "UTCTIMEONLY" => Ok(Self::TimeOnly),
            "NUMINGROUP" => Ok(Self::Int),
            "PERCENTAGE" => Ok(Self::Decimal),
            "SEQNUM" => Ok(Self::Int),
            "TAGNUM" => Ok(Self::Int),
            "LENGTH" => Ok(Self::Int),
            "COUNTRY" => Ok(Self::String),
            "TZTIMEONLY" => Ok(Self::String),
            "TZTIMESTAMP" => Ok(Self::String),
            "XMLDATA" => Ok(Self::String),
            "LANGUAGE" => Ok(Self::String),
            "XID" => Ok(Self::String),
            "XIDREF" => Ok(Self::String),
            "TIME" => Ok(Self::DateTime),
            "DATE" => Ok(Self::String),
            _ => Err(format!("Invalid type: {}", String::from_utf8_lossy(value.as_ref())))
        }
    }
}
