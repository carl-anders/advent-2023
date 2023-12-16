use anyhow::Result;
pub trait Day {
    type Parsed: Clone;
    type Output: ToString;
    fn parse(input: String) -> Result<Self::Parsed>;
    fn first(data: Self::Parsed) -> Self::Output;
    fn second(data: Self::Parsed) -> Self::Output;
}
