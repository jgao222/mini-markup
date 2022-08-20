# Mini-Markup
Mini-markup is a CLI program to convert between a miniature XML-like syntax and XML.

### New Syntax
Write XML and XML adjacent markup while using curly braces to define scopes.
I will refer to the new syntax as mini-XML, or MXML (though it exists already).

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
And back, using `./mini_markup(.exe) -t mxml file2.txt file1.txt`.

Pass the `--help` flag to get a full list of options.

### Why?
I don't particularly like writing HTML by hand, and having to type out closing tag names
seems unnecessary since having the tag name again doesn't add any new meaning.

Curly braces are a familiar syntax for defining scopes, and they can
also be used to define scopes for XML documents, so why not?

#### Notes
1. Using `{}` to delineate blocks means that they can no longer be used as normal characters,
anywhere, and must be escaped with `&lbrkt;` and `&rbrkt;` when in MXML format. The conversion will
automatically handle escaping existing curly brackets in HTML and XML files.
2. Defining scopes/blocks without actual tags before them treats curly braces as literal characters.
So in this case, there is no need to use the escape characters.
For example, something like this `<tag> { some text { no tag? } }` will result in:
`<tag> some text { no tag? } </tag>`.

    However, repeated conversions back and forth will not be identical here, since the standard
MXML form should use `&lbrkt;` and `&rbrkt;` anyways.

3. Converting to and from HTML requires passing the `--html` or `-h` flag to the program since
HTML5 allows [void-element tags](https://html.spec.whatwg.org/multipage/syntax.html#void-elements): tags which are allowed to act like empty-element tags, but
without indicating they are self closing by ending in `/>`. Since these look exactly like
start tags, processing them correctly requires knowing they exists. Please use `-h` when `.html` are involved.