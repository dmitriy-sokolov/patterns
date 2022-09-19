pub mod decorator;

#[cfg(test)]
mod tests {
    use super::decorator::{BasePrice, DiscountDecorator, Price, TaxDecorator};

    #[test]
    fn dynamic_polymorphism() {
        let with_tax = TaxDecorator::new(Box::new(BasePrice));
        let with_tax_and_discount = DiscountDecorator::new(Box::new(with_tax));

        let start_price = 100.0;
        let result = with_tax_and_discount.calculate(start_price);
        assert_eq!(result, 86.25);
    }

    #[test]
    fn static_polymorphism() {
        let start_price = [100.0]; // set start price
        let result = start_price
            .into_iter()
            .map(|e| e * 1.15) // apply tax
            .map(|e| e * 0.75) // apply discount
            .next()
            .unwrap();
        assert_eq!(result, 100.0 * 1.15 * 0.75);
    }
}
