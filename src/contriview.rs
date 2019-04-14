use chrono::*;
use failure::Error;
use juniper::GraphQLObject;
use scraper::*;

#[derive(Debug, Default, PartialEq, Eq, GraphQLObject)]
/// hoge
pub struct ContriView {
    /// hoge
    today_contributions: i32,
    week_contributions: i32,
    month_contributions: i32,
    year_contributions: i32,
    week_ave: i32,
    month_ave: i32,
    sum_ave: i32,
    sum_contributions: i32,
}

impl std::fmt::Display for ContriView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "today_contributions: {}
week_contributions: {}
month_contributions: {}
year_contributions: {}
sum_contributions: {}
week_ave: {}
month_ave: {}
sum_ave: {}",
            self.today_contributions,
            self.week_contributions,
            self.month_contributions,
            self.year_contributions,
            self.sum_contributions,
            self.week_ave,
            self.month_ave,
            self.sum_ave
        )
    }
}

impl ContriView {
    pub fn from_html(html: &str, date: Date<Local>) -> Result<Self, Error> {
        let sum_contributions = Self::sum_contributions_from_html(html);
        let week_contributions = Self::week_contributions_from_html(html);
        let year_contributions = Self::year_contributions_from_html(html, date);
        let month_contributions = Self::month_contributions_from_html(html, date);
        let today_contributions = Self::today_contributions_from_html(html, date);
        let week_ave = week_contributions / 7;
        let month_ave = month_contributions / date.day() as i32;
        let sum_ave = sum_contributions / 365;

        Ok(ContriView {
            sum_contributions,
            week_contributions,
            month_contributions,
            year_contributions,
            today_contributions,
            week_ave,
            month_ave,
            sum_ave,
        })
    }

    fn sum_contributions_from_html(html: &str) -> i32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        input
            .into_iter()
            .map(|i| -> i32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .sum()
    }

    fn month_contributions_from_html(html: &str, date: Date<Local>) -> i32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m").to_string();
        let selector = format!("rect[data-date^=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<i32> = input
            .into_iter()
            .map(|i| -> i32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().sum()
    }

    fn week_contributions_from_html(html: &str) -> i32 {
        let doc = Html::parse_document(&html);
        let selector = Selector::parse(r#"rect[data-date]"#).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<i32> = input
            .into_iter()
            .map(|i| -> i32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().rev().take(7).sum()
    }

    fn year_contributions_from_html(html: &str, date: Date<Local>) -> i32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-").to_string();
        let selector = format!("rect[data-date^=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector);

        let contributions: Vec<i32> = input
            .into_iter()
            .map(|i| -> i32 {
                let contribution = i.value().attr("data-count").unwrap();
                contribution.parse().unwrap_or_default()
            })
            .collect();

        contributions.iter().sum()
    }

    fn today_contributions_from_html(html: &str, date: Date<Local>) -> i32 {
        let doc = Html::parse_document(&html);

        let now = date.format("%Y-%m-%d").to_string();
        let selector = format!("rect[data-date=\"{}\"]", now);

        let selector = Selector::parse(&selector).unwrap();
        let input = doc.select(&selector).next();

        if input.is_none() {
            return 0;
        }

        input
            .unwrap()
            .value()
            .attr("data-count")
            .unwrap()
            .parse()
            .unwrap_or_default()
    }
}
