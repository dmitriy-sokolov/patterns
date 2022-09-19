pub mod decorator;

#[cfg(test)]
mod tests {
    use super::decorator::{BasePrice, DiscauntDecorator, Price, TaxDecorator};

    #[test]
    fn dynamic_polymorphism() {
        let with_tax = TaxDecorator::new(Box::new(BasePrice));
        let with_tax_and_discaunt = DiscauntDecorator::new(Box::new(with_tax));

        let start_price = 100.0;
        let result = with_tax_and_discaunt.calculate(start_price);
        assert_eq!(result, 86.25);
    }
}
