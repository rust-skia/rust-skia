# Copilot Instructions

This file serves as the evolving knowledge base for working with this codebase.
Update it whenever you learn something new about the project's patterns, conventions, or receive feedback that should guide future behavior.

## Project
- Follow the existing project structure and idioms.
- Prefer small, self-contained changes unless explicitly asked for broader refactors.

## Code Style
- Match the surrounding code style.
- Keep functions small, clear, and deterministic.
- Avoid unnecessary dependencies.
- Do not add obvious comments that restate what the code clearly expresses.
- Only comment to explain non-obvious reasoning or intent.
- Limit qualification paths to at most 2 module levels (e.g., `mpsc::channel` not `tokio::sync::mpsc::channel`).
- Import types and modules to reduce path qualification in code.
- Use `pub` visibility by default. Only use `pub(crate)` to limit visibility when the entire containing module is already crate-public.

## Safety & Quality
- Add or update tests when modifying behavior.
- Preserve backwards compatibility unless instructed otherwise.
- When refactoring, don't add trait implementations (Clone, Debug, Default, etc.) that weren't present in the original code.
- If a trait can't be derived due to field constraints, investigate whether the trait is actually needed before implementing it manually.

## Communication
- Explanations should be concise and strictly relevant.
- When unsure, ask clarifying questions before making assumptions.

## Documentation
- Markdown documentation updates to existing files are fine.
- Ask before creating new Markdown documentation files.
- When porting documentation from C++ headers:
    - Stay close to the original wording.
    - Document parameters using a list (e.g. `- param: description`).
    - Link types using brackets (e.g. `[`Type`]`).
    - Use Rust equivalents for C++ types and constants (e.g. `SkPoint` -> `[`Point`]`, `kMove_Verb` -> `[`PathVerb::Move`]`).
    - Do not rename functions when generating documentation.
    - If the Rust function name differs from the C++ function name, use the Rust name in the documentation text.
    - Ensure documentation parameter names match the Rust function parameter names.
