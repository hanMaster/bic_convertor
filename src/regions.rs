use std::collections::HashMap;

pub struct Region {
    regions: HashMap<String, String>
}

impl Region {
    pub fn new() -> Self {
        let mut regions = HashMap::new();
        regions.insert("79".to_string(), "Республика Адыгея".to_string()); //+
        regions.insert("80".to_string(), "Республика Башкортостан".to_string()); //+
        regions.insert("81".to_string(), "Республика Бурятия".to_string()); //+
        regions.insert("84".to_string(), "Республика Алтай".to_string()); // +
        regions.insert("82".to_string(), "Республика Дагестан".to_string()); //+
        regions.insert("26".to_string(), "Республика Ингушетия".to_string()); //+
        regions.insert("83".to_string(), "Кабардино-Балкарская Республика".to_string()); //+
        regions.insert("85".to_string(), "Республика Калмыкия".to_string()); //+
        regions.insert("91".to_string(), "Карачаево-Черкесская Республика".to_string()); //+
        regions.insert("86".to_string(), "Республика Карелия".to_string()); //+
        regions.insert("87".to_string(), "Республика Коми".to_string()); //+
        regions.insert("88".to_string(), "Республика Марий Эл".to_string()); //+
        regions.insert("89".to_string(), "Республика Мордовия".to_string()); //+
        regions.insert("98".to_string(), "Республика Саха (Якутия)".to_string()); //+
        regions.insert("90".to_string(), "Республика Северная Осетия - Алания".to_string()); //+
        regions.insert("92".to_string(), "Республика Татарстан".to_string()); //+
        regions.insert("93".to_string(), "Республика Тыва".to_string()); //+
        regions.insert("94".to_string(), "Удмуртская Республика".to_string()); //+
        regions.insert("95".to_string(), "Республика Хакасия".to_string()); //+
        regions.insert("96".to_string(), "Чеченская республика".to_string()); //+
        regions.insert("97".to_string(), "Чувашская Республика".to_string()); //+
        regions.insert("01".to_string(), "Алтайский край".to_string()); //+
        regions.insert("03".to_string(), "Краснодарский край".to_string()); //+
        regions.insert("04".to_string(), "Красноярский край".to_string()); //+
        regions.insert("05".to_string(), "Приморский край".to_string()); //+
        regions.insert("07".to_string(), "Ставропольский край".to_string()); //+
        regions.insert("08".to_string(), "Хабаровский край".to_string()); //+
        regions.insert("10".to_string(), "Амурская область".to_string()); //+
        regions.insert("11".to_string(), "Архангельская область".to_string()); //+
        regions.insert("12".to_string(), "Астраханская область".to_string()); //+
        regions.insert("14".to_string(), "Белгородская область".to_string()); //+
        regions.insert("15".to_string(), "Брянская область".to_string()); //+
        regions.insert("17".to_string(), "Владимирская область".to_string()); //+
        regions.insert("18".to_string(), "Волгоградская область".to_string()); //+
        regions.insert("19".to_string(), "Вологодская область".to_string()); //+
        regions.insert("20".to_string(), "Воронежская область".to_string()); //+
        regions.insert("24".to_string(), "Ивановская область".to_string()); //+
        regions.insert("25".to_string(), "Иркутская область".to_string()); //+
        regions.insert("27".to_string(), "Калининградская область".to_string()); //+
        regions.insert("29".to_string(), "Калужская область".to_string()); //+
        regions.insert("30".to_string(), "Камчатский край".to_string()); //+
        regions.insert("32".to_string(), "Кемеровская область".to_string()); //+
        regions.insert("33".to_string(), "Кировская область".to_string()); //+
        regions.insert("34".to_string(), "Костромская область".to_string()); //+
        regions.insert("37".to_string(), "Курганская область".to_string()); //+
        regions.insert("38".to_string(), "Курская область".to_string()); //+
        regions.insert("41".to_string(), "Ленинградская область".to_string()); //+
        regions.insert("42".to_string(), "Липецкая область".to_string()); //+
        regions.insert("44".to_string(), "Магаданская область".to_string()); //+
        regions.insert("46".to_string(), "Московская область".to_string()); //+
        regions.insert("47".to_string(), "Мурманская область".to_string()); //+
        regions.insert("22".to_string(), "Нижегородская область".to_string()); //+
        regions.insert("49".to_string(), "Новгородская область".to_string()); //+
        regions.insert("50".to_string(), "Новосибирская область".to_string()); //+
        regions.insert("52".to_string(), "Омская область".to_string()); //+
        regions.insert("53".to_string(), "Оренбургская область".to_string()); //+
        regions.insert("54".to_string(), "Орловская область".to_string()); //+
        regions.insert("56".to_string(), "Пензенская область".to_string()); //+
        regions.insert("57".to_string(), "Пермский край".to_string()); //+
        regions.insert("58".to_string(), "Псковская область".to_string()); //+
        regions.insert("60".to_string(), "Ростовская область".to_string()); //+
        regions.insert("61".to_string(), "Рязанская область".to_string()); //+
        regions.insert("36".to_string(), "Самарская область".to_string()); //+
        regions.insert("63".to_string(), "Саратовская область".to_string()); //+
        regions.insert("64".to_string(), "Сахалинская область".to_string()); //+
        regions.insert("65".to_string(), "Свердловская область".to_string()); //+
        regions.insert("66".to_string(), "Смоленская область".to_string()); //+
        regions.insert("68".to_string(), "Тамбовская область".to_string()); //+
        regions.insert("28".to_string(), "Тверская область".to_string()); //+
        regions.insert("69".to_string(), "Томская область".to_string()); //+
        regions.insert("70".to_string(), "Тульская область".to_string()); //+
        regions.insert("71".to_string(), "Тюменская область".to_string()); //+
        regions.insert("73".to_string(), "Ульяновская область".to_string()); //+
        regions.insert("75".to_string(), "Челябинская область".to_string()); //+
        regions.insert("76".to_string(), "Забайкальский край".to_string()); //+
        regions.insert("78".to_string(), "Ярославская область".to_string()); //+
        regions.insert("45".to_string(), "Москва".to_string()); //+
        regions.insert("40".to_string(), "Санкт-Петербург".to_string()); //+
        regions.insert("67".to_string(), "Севастополь".to_string()); //+
        regions.insert("99".to_string(), "Еврейская автономная область".to_string()); //+
        regions.insert("35".to_string(), "Республика Крым".to_string()); //+
        regions.insert("111".to_string(), "Ненецкий автономный округ".to_string()); //+
        regions.insert("118".to_string(), "Ненецкий автономный округ".to_string()); //+
        regions.insert("71100".to_string(), "Ханты-Мансийский автономный округ — Югра".to_string()); //+
        regions.insert("718".to_string(), "Ханты-Мансийский автономный округ — Югра".to_string()); //+
        regions.insert("77".to_string(), "Чукотский автономный округ".to_string()); //+
        regions.insert("719".to_string(), "Ямало-Ненецкий автономный округ".to_string()); //+
        regions.insert("71140".to_string(), "Ямало-Ненецкий автономный округ".to_string()); //+

        Self {
            regions
        }
    }

    pub fn get_by_code(&self, code: &str) -> Result<&String, &'static str> {
        self.regions.get(code).ok_or("region not found")
    }
}