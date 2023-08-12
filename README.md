# trait-annotations

[Chapter 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html) from [The Rust Book](https://doc.rust-lang.org/book/) introduces the concept of **trait**, which defines a functionality a type has and can share with others. Given that traits are a fundamental feature of the language for providing shared behaviour (along with generics), this small project serves as a more detailed analysis of the tool for a better comprehension of its utility.

# Coherence and the orphan rule

Implementing traits for types in Rust consider some implicit rules that guarantee both safety and correctness while using them. The first of these rules are the idea of **trait coherence**, which states that, given a trait and some set of types for its type parameters, there should be exactly one `impl` block that applies. So for a trait like `Display`, it should be guaranteed that if a trait reference like `MyType : Display` is identified, it can only be mapped to a particular implementation.

Additionally, the **orphan rule** is basically used to prevent users from implementing external traits for external types. As an example, in the context of a user's own library, implementing `Display` for `Vec<T>` would not be possible because both are defined in the standard library. However, `Display` could be implemented for a user-defined type `MyType` and a user-defined trait `MyTrait` can be implemented for any type, including external ones. Shortly put, according to this rule either the trait must be local or the self-type must be local.

In a practical scenario, without these rules, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.