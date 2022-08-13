# Mini-Markup
Mini-markup is a CLI program to convert a minified XML-like syntax into XML.

It currently only works to convert in that one direction.

### New Syntax
Write this:
```xml
<tagName attribute="value"> {
    <childTag> {
        Content goes here!
    }
}
```
and automatically convert it to this:
```xml
<tagName attribute="value">
    <childTag>
        Content goes here!
    </childTag>
</tagName>
```

### Why?
I don't particularly like writing HTML by hand, and having to type out closing tag names
seems unnecessary since having the tag name again doesn't add any new meaning.

Curly braces are a familiar syntax for defining scopes, and (with a few caveats) they can
also be used to define scopes for XML documents.

#### Those caveats
1. Using `{}` to delineate blocks means that they can no longer be used as normal characters,
anywhere, and must be escaped with `&lbrkt;` and `&rbrkt;`.
2. Defining scopes/blocks without actual tags before them currently leads to undefined behavior.
For example, don't to something like this `<tag> { some text { no tag? } }`.
