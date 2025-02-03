## Word Generator

This is a generator for words that are supposed to look somewhat similar. This can be useful to generate gibberish that mimics real languages. There are still plently of improvements to be made to the generator and it is currently nothing more than an MVP.

This document provides an overview of the program and how it can be used.

### Functionality

The main reasoning behind the functionality is determinism. This means that the whole program is based on a single seed and strictly defined rules. The rules are defined in a YAML file and read as a binary tree map, more on this in the following [section](#data).

The following conecepts are key in the algorithm:
- **candidate**: one of usually several candidates for the generated word. Only one candidate is returned, based on its value.
- **value**: the value of how well a candidate complies with the language rules. The value is calculated as the sum of the proximity to the average word length [0, 1] and the normalized termination weight of its pattern [0, 1].
- **pattern**: the currently last *n* letters of a word candidate. This is used to find possible continuations.
- **continuation**: the letter(s) that can be appended to a word candidate to create another word candidate.
- **weight**: the relative chance for each continuation to be used. The real (normalized) chance is intuitively $\frac{weight_{continuation}}{\Sigma weights}$.
- **wildcards**: the continuations can contain the special wildcard symbol ``_``. This gets replaced by any letter from the ``alphabet``, except for any continuation already defined for that pattern.

### <a name="data"></a> Data Structures

The algorithm uses a *patterns map* when generating new words. This is different from, although similar to, the *rules map*, which is used to define a particular language. All maps are binary tree maps.

The *rules map* is defined in a YAML file ([example](assets/examples/example.yaml)) and is used to generate the *patterns map* at runtime. The *rules map* can also be generated from a file containing plain text, more on this in [CLI usage](#cli).

The format of the *rules map* is ``Map<String, Map<String, Int>>``. Each pattern string maps to its own map for possible continuations and their respective relative weights. There are two unique patterns ``alphabet`` and ``word_length``. The map in ``alphabet`` contains a single key-value pair, where the value is a string of all unique letters available in the language, and the value is irrelevant (0 by default). The map in ``word_length`` contains 3 key-value pairs, ``min``, ``avg``, and ``max``, which map to the minimum word length, average word length, and maximum word length respectively.

Apart from the two unique patterns, there are no limitations to what patterns can exist, although they must contain only letters present in the ``alphabet``. This, along with the correct format of the unique patterns, is verified at runtime before generating words. If the format is incorrect, the program panics with a descriptive error message.

The format of the *patterns map* is ``Map<String, (Int, Float, Map<Int, String>)>``. Each pattern string maps to a three-value tuple. The first value is the highest allowed value for the RNG when picking a continuation [1, 4294967295]. The second value is the normalized termination weight of that pattern [0, 1]. The third value is the internal map, similar to that of the *rules map*, but the key-value pairs are flipped and the relative weight is cumulative.

### Public API

The public API consists of the following modules:
- ``command``: use when working with application arguments to define the *rules map*. ``get_rules`` returns a *rules map*.
- ``reader``: use when generating the *rules map* based on a sample text. ``rules_from_string`` returns a *rules map*.
- ``verification``: is called before generating words to verify the *rules map* is in the correct format. ``verify_rules`` returns a ``Result``; ``Ok()`` if passes, ``Err(String)`` if fails.
- ``generator``: generates words, given a *rules map* and amount. ``generate_words`` returns a ``String`` with that amount of words, separated by single spaces.

### <a name="cli"></a>CLI Usage

The program can be easily run with ``cargo``.
- ``cargo run``: running without arguments causes failure. There are two arguments that can be used separately. Providing both arguments is illogical, and gives precedence to ``s``.
    - ``s, sample-text``: Used when providing the path of a sample text file. The file can be in any format and the ``reader`` ignores all non-alphabetic characters. Overwrites ``assets/local/rules.yaml`` with the newly created language rules. Example: ``cargo run -- -s english.txt``.
    - ``r, language-rules``: Used when providing pre-made rules in the correct [*rules map* format](#data). Example ``cargo run -- -r rules.yaml``.
    - NOTE: when only providing the file name, the file must reside in ``assets/local`` or ``assets/examples``, otherwise the full path is needed.
    - NOTE: it is recommended to use sample texts with at least several thousand words for decent results.
- ``cargo test``: runs all unit and integration tests.
