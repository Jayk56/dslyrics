# Lyrics Domain-Specific Language (DSL)

**Version 1.0**

## Introduction

The Lyrics Domain-Specific Language (DSL) is a formal language for representing, parsing, validating, and grading song lyrics according to music industry standards. Designed for songwriters, producers, music educators, and developers, the DSL enables:

- **Structural validation** of song components (verses, choruses, etc.)
- **Prosodic analysis** (meter, rhyme, and rhythm)
- **Industry-standard grading** for commercial viability
- **Multi-language support** with localization constraints
- **Integration points** for musical notation and timing

This repository implements the Lyrics DSL as specified below, including grammar, validation pipeline, grading, and extensibility recommendations.

---

## DSL Grammar (EBNF)

The core structure and syntax of a song in the Lyrics DSL is defined as follows:

```ebnf
(* Top-level structure *)
song            = metadata sections EOF ;
metadata        = meta_entry+ ;
meta_entry      = meta_key ":" meta_value NL ;
sections        = section+ ;
section         = verse | chorus | bridge | pre_chorus | outro | intro ;

(* Metadata keys *)
meta_key        = "title" | "artist" | "tempo" | "key" | "time_sig" | 
                  "genre" | "lang" | "writers" | "duration" ;
meta_value      = STRING | NUMBER | identifier ;

(* Section definitions *)
verse           = "VERSE" section_number? section_attrs? NL lines ;
chorus          = "CHORUS" section_number? section_attrs? NL lines ;
bridge          = "BRIDGE" section_attrs? NL lines ;
pre_chorus      = "PRE-CHORUS" section_attrs? NL lines ;
outro           = "OUTRO" section_attrs? NL lines ;
intro           = "INTRO" section_attrs? NL lines ;

section_number  = "[" NUMBER "]" ;
section_attrs   = "{" attr_list "}" ;
attr_list       = attribute ("," attribute)* ;
attribute       = attr_name ":" attr_value ;
attr_name       = identifier ;
attr_value      = STRING | NUMBER | boolean ;

(* Line structure *)
lines           = line+ ;
line            = line_content line_attrs? NL ;
line_content    = TEXT ;
line_attrs      = "{" line_attr_list "}" ;
line_attr_list  = line_attribute ("," line_attribute)* ;
line_attribute  = "rhyme" ":" rhyme_scheme |
                  "stress" ":" stress_pattern |
                  "chord" ":" chord_sequence |
                  "timing" ":" timing_info ;

(* Primitives *)
TEXT            = /[^\n{]+/ ;
STRING          = '"' /[^"]*/ '"' ;
NUMBER          = /[0-9]+(\.[0-9]+)?/ ;
identifier      = /[a-zA-Z_][a-zA-Z0-9_]*/ ;
boolean         = "true" | "false" ;
rhyme_scheme    = /[A-Z]/ ;
stress_pattern  = /[x\/]+/ ;
chord_sequence  = chord ("," chord)* ;
chord           = /[A-G][#b]?(maj|min|dim|aug|[0-9]+)?/ ;
timing_info     = NUMBER ":" NUMBER ;
NL              = "\n" ;
EOF             = end of file ;
```

---

## Validation Rules

### Structural Constraints

```yaml
structural_rules:
  song_structure:
    - must_have_metadata: true
    - min_sections: 2
    - max_sections: 20
    - required_sections: ["verse", "chorus"]
    
  section_rules:
    verse:
      min_lines: 4
      max_lines: 8
      max_count: 5
    chorus:
      min_lines: 2
      max_lines: 6
      max_count: 5
    bridge:
      min_lines: 2
      max_lines: 8
      max_count: 1
    pre_chorus:
      min_lines: 2
      max_lines: 4
      max_count: 3
```

### Prosodic Constraints

```yaml
prosodic_rules:
  syllable_constraints:
    chorus_line:
      max_syllables: 8
      variance_allowed: 1
    verse_line:
      max_syllables: 12
      variance_allowed: 2
      
  rhyme_patterns:
    accepted_schemes:
      verse: ["AABB", "ABAB", "ABCB", "AAAA"]
      chorus: ["AABB", "AAAA", "ABAB"]
    
  meter_patterns:
    common_meters:
      - iambic_pentameter: "x/x/x/x/x/"
      - trochaic_tetrameter: "/x/x/x/x"
      - anapestic_trimeter: "xx/xx/xx/"
```

### Musical Constraints

```yaml
musical_rules:
  tempo_ranges:
    ballad: [60, 80]
    pop: [100, 130]
    dance: [120, 140]
    
  chord_progressions:
    must_resolve: true
    parallel_fifths: forbidden
    max_complexity: 4  # chords per bar
```

---

## Validation Pipeline

The Lyrics DSL system uses a three-stage pipeline: parsing, linting, and grading.

### Parser Stage

```typescript
interface Parser {
  parse(input: string): AST | ParseError;
}

interface AST {
  type: "song";
  metadata: Metadata;
  sections: Section[];
}
```

### Linter Stage

```typescript
interface Linter {
  lint(ast: AST): LintResult[];
}

interface LintResult {
  severity: "error" | "warning" | "info";
  rule: string;
  message: string;
  location: Location;
}
```

### Grader Stage

```typescript
interface Grader {
  grade(ast: AST, lintResults: LintResult[]): Grade;
}

interface Grade {
  overall: number;  // 0-100
  breakdown: {
    structure: number;
    prosody: number;
    originality: number;
    commerciality: number;
  };
  feedback: string[];
}
```

---

## Industry-Standard Grading Criteria

- **Structure Score (25%)**
  - Section count and order, line consistency, hooks, and dynamic arc.
- **Prosody Score (25%)**
  - Rhythm, word stress, rhyme, and syllable count.
- **Originality Score (25%)**
  - Unique phrases, fresh imagery, avoidance of clichés, creative rhyme.
- **Commercial Viability Score (25%)**
  - Singability, emotional resonance, universal themes, radio-friendliness.

---

## Example

### Valid Song Input

```
title: "Validation Blues"
artist: "The Parsers"
tempo: 110
key: "C"
genre: "pop"

VERSE[1]
Walking through the syntax tree {rhyme: A}
Every node must be just right {rhyme: B}
Counting syllables carefully {rhyme: A}
Making sure the meter's tight {rhyme: B}

CHORUS
Validate, validate {rhyme: A}
Every single line {rhyme: B}
Parse it till it's perfect {rhyme: C}
Everything's in time {rhyme: B}

VERSE[2]
Error messages guide the way {rhyme: A}
Red squiggles show what's wrong {rhyme: B}
Fix them all without delay {rhyme: A}
Now the structure's strong {rhyme: B}

BRIDGE
When the linter's happy {rhyme: A}
And the grade is high {rhyme: B}
Ship it to production {rhyme: C}
Watch your lyrics fly {rhyme: B}

CHORUS
Validate, validate {rhyme: A}
Every single line {rhyme: B}
Parse it till it's perfect {rhyme: C}
Everything's in time {rhyme: B}
```

### Example Validation Output

```json
{
  "valid": true,
  "warnings": [
    {
      "rule": "syllable_variance",
      "message": "Line syllable counts vary by more than 2",
      "location": "VERSE[2]:line3"
    }
  ],
  "grade": {
    "overall": 85,
    "breakdown": {
      "structure": 90,
      "prosody": 85,
      "originality": 75,
      "commerciality": 90
    },
    "feedback": [
      "Strong ABAB rhyme scheme throughout",
      "Consider varying the rhythm in the bridge",
      "Chorus is highly memorable and singable"
    ]
  }
}
```

---

## Implementation Recommendations

- **Parser**: ANTLR4 or Tree-sitter recommended. PEG.js for web; hand-written parser for lightweight use.
- **Syllable Counting**: Use CMU Pronouncing Dictionary or fallback estimates.
- **Rhyme Detection**: Extract phonetic endings of words.
- **Stress Pattern Analysis**: Map syllable stress using phonetic data.

---

## Extension Points

- **Plugin Architecture**:  
  Custom rules, validators, or graders can be added via plugin interface.
- **Custom Rules**:  
  Support for genre/language/culture/artist-specific constraints.
- **Integration APIs**:  
  MIDI export, chord chart, karaoke, and music notation sync.

---

## Error Messages

- **Parse Errors**:  
  `ERROR: Expected CHORUS after VERSE[2] at line 15`
- **Validation Errors**:  
  `ERROR: Chorus exceeds maximum 6 lines (found 8)`
- **Grade Feedback**:  
  `INFO: Structure score reduced: missing pre-chorus creates abrupt transition`

---

## Future Enhancements

- **V2.0 Features**:  
  AI-powered lyric suggestions, melody/pitch integration, real-time collaboration, multi-language rhyme dictionaries, semantic validation.
- **Research Directions**:  
  Emotion modeling, cross-cultural prosody, neural syllable counting, genre classification, plagiarism detection.

---

## Song Structure Patterns

**Pop Song Structure**
```
INTRO (optional)
VERSE[1]
CHORUS
VERSE[2]
CHORUS
BRIDGE
CHORUS
OUTRO (optional)
```

**Ballad Structure**
```
VERSE[1]
VERSE[2]
CHORUS
VERSE[3]
CHORUS
BRIDGE
CHORUS (modulated)
```

---

## Reference Implementation

A full TypeScript reference implementation is available at:  
[github.com/lyrics-dsl/reference-implementation](https://github.com/lyrics-dsl/reference-implementation)

Key modules:
- Parser: `src/parser/`
- Linter: `src/linter/`
- Grader: `src/grader/`
- CLI: `src/cli/`
- VS Code Extension: `extensions/vscode/`

---

## License

MIT License

---
