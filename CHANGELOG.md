# Changelog

## 0.3.0 [2021-07-07]

* [BUG]: Fix issue with tabs. Tabs will now be replaced by a single space. That
makes it easier to reuse the existing formatting infastructure.
* [BUG]: Fix off by one error with columns in yaml errors. The indicator where
the error occured was one too much to the right.

## 0.2.0 [2021-06-07]

* Contextualize long lines in the output [#1]
* Add ways to change the amount of context shown for lines and characters [#8].
* Add crate features [#2, #6]

## 0.1.0 [2021-05-31]

* Support formatting of `serde_json` and `serde_yaml`.
