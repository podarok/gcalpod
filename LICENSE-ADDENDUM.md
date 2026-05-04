# gcalpod Sustainable License Addendum v1 (gSL-v1)

This **addendum** sits on top of [`LICENSE`](LICENSE) (PolyForm
Noncommercial 1.0.0). It grants **additional permissions** beyond
those in the core PolyForm Noncommercial license. It does **not**
remove or weaken any term of the core license; if a conflict arises
the addendum's grants are construed narrowly and the core license
controls.

Use of this software constitutes acceptance of both the core license
and this addendum.

---

## A. Sponsorship Tier Grant

If **You** (as defined in the core license) maintain an active,
recurring sponsorship to the licensor at **USD $5/month or higher**
through any of:

- [GitHub Sponsors](https://github.com/sponsors/podarok)
- [Patreon](https://www.patreon.com/podarok_ua)
- [Buy Me a Coffee](https://www.buymeacoffee.com/podarok)
  (recurring tier ≥ $5/mo)

then **You and Your Company** receive an automatic, non-exclusive,
worldwide, royalty-free license to use the software for **any
purpose**, including commercial purposes, for the period during
which the sponsorship is active and for thirty (30) days thereafter.

The grant ends if sponsorship lapses; You then have ninety (90)
days to either resume sponsorship, negotiate a separate commercial
license, or revert to noncommercial use under the core license.

## B. Solo / Micro Carveout

You may use the software for any commercial purpose without a
separate commercial license if **all** of the following are true
about Your Company at the time of use:

1. **Headcount** — two (2) or fewer total employees, contractors,
   and equity-holding founders combined; **and**
2. **Revenue** — less than USD $20,000 in gross worldwide revenue
   in the most recent completed fiscal year (or, if Your Company is
   newer than one fiscal year, in the trailing twelve months); **and**
3. **No funding cliff** — Your Company has not raised more than USD
   $20,000 in total external equity funding (venture capital, angel
   rounds, PE, etc.); grants and revenue do not count.

Sole proprietors, freelancers, self-employed contractors, and
hobbyists meeting the above thresholds are explicitly covered.

Once any threshold is exceeded, You have ninety (90) days to either
sponsor at the Addendum A tier, negotiate a separate commercial
license, or stop using the software for commercial purposes.

You must self-assess in good faith. The licensor reserves the right
to ask, in writing, for confirmation; You agree to respond within
thirty (30) days.

## C. Apache 2.0 Time-Bomb (Anti-Lock-In)

Each tagged release version of the software (e.g. `v1.0.0`,
`v1.1.0`, ...) **automatically becomes available under the [Apache
License, Version 2.0](LICENSE-Apache-2.0)** four (4) years after
that version's tag date.

This grant covers only the version that has reached its
four-year anniversary; subsequent versions remain under the core
license + this addendum until they reach their own anniversary.

Verification: the tag date for any release is the
`gh release view <tag> --json publishedAt` value or the equivalent
git-tag committer timestamp.

## D. Package Distribution Carveout

The following entities may distribute, package, mirror, and
catalogue the software (including modified or trimmed forms thereof)
without a separate commercial license, regardless of any commercial
nature of the distribution channel itself:

- Operating-system package repositories (Homebrew, MacPorts, AUR,
  Debian/Ubuntu, Fedora/RHEL, NixOS, Gentoo, Alpine, FreeBSD ports,
  Termux, Chocolatey, winget, Scoop, etc.).
- Language ecosystem registries (crates.io, PyPI, npm, RubyGems,
  Maven Central, Hex, Packagist).
- Software discovery and mirror services (GitHub, GitLab,
  Codeberg, sr.ht, archive mirrors, software heritage).

This grant covers only the act of packaging, hosting, mirroring,
and serving downloads. It does **not** grant downstream users any
right beyond what they already have under the core license + the
rest of this addendum.

## E. Contribution Grant

Contributors who submit patches, pull requests, issues, design
proposals, or other materials accepted into the software:

1. License their contributions to the licensor under the
   core PolyForm Noncommercial 1.0.0 license, plus this addendum,
   without requiring a separate Contributor License Agreement (CLA).
2. Receive an automatic, non-exclusive, royalty-free, worldwide
   license to use the software (including their contribution) for
   **any purpose, including commercial use** for as long as they
   retain authored commits in the project's git history.

This grant is intentionally generous to encourage contribution:
maintainers and contributors should never be in a worse position
than non-contributing users.

## F. Good-Faith Interpretation

Where this addendum or the core license is ambiguous, the parties
agree to interpret terms in a manner consistent with the spirit of
the license:

- the licensor wants to be paid for commercial value extracted by
  well-resourced users;
- the licensor does **not** want to chill hobbyists, students,
  contributors, packagers, charities, or independent professionals;
- the licensor does **not** want lock-in fear to block adoption.

If the strictest reading of a term would harm someone the licensor
clearly intends to support, the more permissive reading prevails.

---

## How to comply (quick guide)

| If you are... | What applies | What you owe |
|---|---|---|
| Hobbyist / student / employee on personal machine | core license (Noncommercial / Personal Uses) | nothing |
| Solo or 2-person micro-business, < $20k revenue, ≤ $20k raised | Addendum B | self-assess; nothing |
| Contributor with merged commits | Addendum E | nothing; you have commercial rights |
| Larger company, sponsoring ≥ $5/mo | Addendum A | maintain sponsorship |
| Larger company, no sponsorship | core license (Noncommercial only) | negotiate commercial license, or sponsor, or wait 4 years (Addendum C) |
| Distro / registry packager | Addendum D | preserve the LICENSE + LICENSE-ADDENDUM files |

## Governance

- This addendum may evolve; the version of the addendum bundled with
  a tagged release applies to that release. Subsequent addendum
  versions never apply retroactively.
- Disputes are subject to good-faith resolution first
  (Addendum F). Litigation is a last resort.

## Contact for commercial license

If none of the above paths fit Your situation, request a custom
commercial license: open a private email thread via the GitHub
repository's Discussions / Issues, or contact the licensor through
the sponsorship platforms listed in [`.github/FUNDING.yml`](.github/FUNDING.yml).
