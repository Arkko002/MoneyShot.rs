use crossterm::terminal;

use crate::services::models::TradesReport;

pub fn pretty_print_reports(reports: Vec<TradesReport>, print_stats: bool) {

    let (x,y) = terminal::size().unwrap();

    let delimeter = "-".repeat(x.into());
    println!("{}", delimeter);

    for report in reports.iter() {
        println!(
            "Operation: {:#?} | Price: {} | Volume: {}",
            report.operation, report.price, report.volume
        );
    }

    if print_stats {
        pretty_print_stats(reports)
    };

    println!("{}", delimeter);
}

pub fn pretty_print_stats(reports: Vec<TradesReport>) {
    println!("Report Number: {:?}", reports.len());
}
