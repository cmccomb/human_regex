# Changelog

## 0.3.1

### Fixes

- Escape literal endpoints in `within()` and `without()`. Ranges containing
  brackets, backslashes, hyphens, or carets now match the intended characters.
- Escape every regex metacharacter in `escape_all()`, including periods, plus
  signs, parentheses, and backslashes.
- Correct non-capturing groups in `or()` and `and()` (including the `&` operator).
  These expressions no longer accept an unintended leading colon or introduce
  an implicit capture group.

### Compatibility notes

The public API remains the 0.3.x API. The type-system changes and renamed
functions on `master` are not part of this release.

Correcting `or()` and `and()` removes their accidental capture groups. Numbered
capture indices and numeric replacement references can therefore change in
expressions that use them. Use `capture()` or `named_capture()` explicitly where
a capture is needed, and review numeric references when upgrading.

`escape_all()` now treats all supplied strings as literals. Pass intentional
regex fragments directly to `or()` instead of passing them through `escape_all()`.

### Documentation and maintenance

- Correct uppercase/lowercase mappings, lazy-repetition examples, and
  distinctions between ASCII and Unicode character classes.
- Document automatic escaping of range endpoints and complete literal escaping.
- Remove an unsupported stable-rustdoc lint.
- Run CI for the `release/0.3` maintenance branch as well as `master`.
- Update the CI checkout action from v2 to v7.
- Check the minimum supported `regex` dependency version (1.7.1) in CI.
- Add regression tests for escaping, range membership, and explicit captures.
