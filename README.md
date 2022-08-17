# Mini-Markup
Mini-markup is a CLI program to convert between a miniature XML-like syntax and XML.

### New Syntax
Write this:
```xml
<!-- file1.txt -->
<tagName attribute="value"> {
    <childTag> {
        Content goes here!
    }
}
```
and call `./mini_markup(.exe) file1.txt file2.txt` to convert it to this:
```xml
<!-- file2.txt -->
<tagName attribute="value">
    <childTag>
        Content goes here!
    </childTag>
</tagName>
```
And back, if desired. Pass the `--help` flag to get a full list of options.

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
