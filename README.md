# DoomAlerts
[![Build & Test](https://github.com/ro6ley/DoomAlerts/actions/workflows/test.yml/badge.svg)](https://github.com/ro6ley/DoomAlerts/actions/workflows/test.yml)

A bot built in Rust to notify me in advance if KPLC is scheduled to do full-day maintenance in my area.

## Modus Operandi

KenyaPower usually posts planned power interruption notifications on their [Twitter page (@KenyaPower_Care)](https://twitter.com/kenyapower_care) and [their website](https://www.kplc.co.ke/category/view/50/planned-power-interruptions). The frequency of notifications posted
varies between Twitter and the website, with their Twitter page being the most up to date.

Planned power interruption notifications posted on their Twitter page are usually posted the day before the scheduled interruption. They are posted as tweets that contain one or more images that contain the interruption details such as areas affected, date of the planned interruption e.t.c. Samples of the images can be found in the `tests/images` folder in this project, also see [Samples section below](#samples).

DoomAlerts bot periodically fetches these tweets and extracts the power interruption information from these images using OCR.

Once the interuption information is extracted from these images, a search is done to look for the locations configured by the `WATCHLIST` environment variable. If an area on the watchlist is found within the text extracted from the images, an email is sent to the email address configured by the `EMAIL_RECIPIENT` environment variable.

### Samples

Input sample image: ![](./tests/images/test_2.png)

OCR'd text:
```
AREA: WHOLE OF UTAWALAFEEDER, DATE: Tuesday 08.03.2022 TIME: 9.00 A.M.—5.00P.M.

Parts of Eastern Bypass, Ruai Tuskys Supermarket, Triple O Hotel, Bakri Petrol Stn, Fahari Hotel, Komarock Medical Services Hosp, St. Bhakita Hosp, Oil Libya
Petrol Stn, Benedicta Hosp, Parts of Mihango Est, Utawala Shopping Centre, Kinka Est, Parts of Githunguri Rd, Maji ya Ngilu, Tamarind Estate, Zebra Est, MC
Estate & adjacent customers.

AREA: KITUI ROAD DATE: Tuesday 08.03.2022 TIME: 9.00 A.M.- 5D0P.M.

Legend, Kyeni, Kaseve, Kalumoni, Kaani, Kinthangathini, Vyulya, Masii, Muthetheni, Makutano Mwela, Mwala, Kivandini, Mbiuni, Yathui, kalaase, Tulila,
Wamunyu, Katangi, Syokisinga, Ikombe, Seku University, Kyua & adjacent customers.

AREA: HIRUMBI MARKET. DATE: Tuesday 08.03.2022 TIME: 9.00 A.M.—2.00P.M.

Bukhulunya Mkt, Hirumbi Mkt & adjacent customers.

AREA: MAKONGENI, GOT RABUOR, NGEGU DATE: Tuesday 08.03.2022 TIME: 10.00 A.M. - 3.00 P.M.

Ohero, Omoya, Akili Pri Sch, Baracuda Hotel, Lwaho Pri Sch, Mariwa, Ndiru Mkt, Omoche, Manyatta, Luore, God Ponge, Onyege Mkt & adjacent customers.
```


## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Building Blocks

- Rust
- Tesseract
- [Leptess v0.13.2](https://github.com/houqp/leptess)
- [Tantivy](https://github.com/quickwit-oss/tantivy)

### Environment Variables

The following environment variables are needed to be set when running the bot:

* `API_KEY` - as provided by Twitter on the developer portal, configuring a Twitter App as a Social App will provide it
* `API_SECRET_KEY` - as provided by Twitter on the developer portal, configuring a Twitter App as a Social App will provide it
* `WATCHLIST` - comma separated list of areas to be on the lookout for
* `EMAIL_USERNAME`
* `EMAIL_PASSWORD`
* `EMAIL_SMTP_HOST`
* `EMAIL_RECIPIENT`
* `INTERVAL` - fetch tweets every `INTERVAL` seconds


### Local Development

* clone the repo:
  ```bash
  $ git clone https://github.com/ro6ley/DoomAlerts.git
  $ cd DoomAlerts
  ```

* create the `.env` file and fill in the required details:
  ```bash
  $ cp .env.example .env
  ```

* build:
  ```bash
  $ cargo install --path .
  ```

* start the application and run it in the background:
  ```bash
  $ source .env
  $ doom_alerts &
  ```

* run tests:
  ```bash
  $ cargo test
  ```

## Documentation

To view the project's documentation run:
```bash
$ cargo doc --no-deps --open
```

### Docker

To run DoomAlerts using Docker:

* Build the image:
  ```bash
  $ docker build -t doom_alerts .
  ```

* Run it:
  ```bash
  $ docker run --env-file .env doom_alerts
  ```
