# Advent of Code 2024 in Rust
This repository is being prepared for Advent of Code 2024. Below are the personal rules and themes I plan to follow starting December 1, 2024:

* AI Assistance: Leveraging AI tools is highly encouraged (see the `Why AI?` section below).
* Primary Goals:
  * Solve problems quickly — prioritize functionality over perfection or optimization (unless performance becomes a bottleneck).
  * Emphasize robust testing — relying on AI and working at a greater speed increases the risk of introducing bugs. Good tests will help quickly identify issues, keeping the feedback loop tight and efficient.

# Why AI?
For Advent of Code 2024, I plan to heavily rely on AI to streamline problem-solving. In previous years, I focused on maximizing learning and manual problem-solving, but this approach comes with notable downsides:

* Time - Solving problems manually takes significantly longer.
* Focus & Motivation - Prolonged problem-solving can lead to burnout or loss of interest in what is ultimately a fun, optional side project.

## Pros
By integrating AI into the process, I aim to address these challenges while taking advantage of several key benefits:

### Clarity Through Explanation
Explaining a problem or solution to an AI can be as valuable as solving it yourself. This mirrors the concept of Rubber Duck Debugging, where articulating your thoughts helps uncover insights or errors.

### Efficient Rust Code Generation
Collaborating with AI to write Rust code is both productive and enjoyable. My planned approach includes:
* Defining a rough set of types that would be used to solve the problem manually.
  * AI can assist in generating these types. Sometimes it’s faster to describe the desired types or use familiar shorthand notation for the AI to translate than writing them out yourself.
* Iteratively describing implementation details to the AI, one function at a time, using these types as a guide.
* Breaking problems into smaller parts when the AI's output doesn’t align with expectations.
* As a last resort, writing code manually — but even then, using AI to quickly find relevant documentation and information.

## Cons
While AI can be a powerful tool, it is not without its drawbacks:

### Over-reliance
Depending too much on AI can hinder personal growth and understanding of the problem-solving process, which in turn makes it more difficult to know when the AI has synthesized problematic code.
### Debugging Complexity
Understanding and debugging AI-generated code can sometimes be more challenging than code written manually.
### Ethical Considerations
Using AI tools raises questions about the originality and ownership of the generated code. This is a problem we need to tackle as a society since AI tools are likely here to stay, but as of now we don't yet have a solution.

# Conclusion
It seems plausible that the future of software development will lean increasingly on AI in order to get more done in less time. For this reason, and after weighing the pros vs cons, AI will be a big part of solving this year's Advent of Code problems for me.
