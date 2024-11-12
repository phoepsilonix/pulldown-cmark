// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn definition_lists_test_1() {
    let original = r##"apple
:   red fruit

orange
:   orange fruit
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>red fruit</dd>
<dt>orange</dt>
<dd>orange fruit</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_2() {
    let original = r##"apple

:   red fruit

orange

:   orange fruit
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>
<p>red fruit</p>
</dd>
<dt>orange</dt>
<dd>
<p>orange fruit</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_3() {
    let original = r##"apple

:   red fruit
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>
<p>red fruit</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_4() {
    let original = r##"apple
  : red fruit

orange
  : orange fruit
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>red fruit</dd>
<dt>orange</dt>
<dd>orange fruit</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_5() {
    let original = r##"apple

 : red fruit

orange

 : orange fruit
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>
<p>red fruit</p>
</dd>
<dt>orange</dt>
<dd>
<p>orange fruit</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_6() {
    let original = r##"*apple*

:   red fruit

    contains seeds,
    crisp, pleasant to taste

*orange*

:   orange fruit

        { orange code block }

    > orange block quote
"##;
    let expected = r##"<dl>
<dt><em>apple</em></dt>
<dd>
<p>red fruit</p>
<p>contains seeds,
crisp, pleasant to taste</p>
</dd>
<dt><em>orange</em></dt>
<dd>
<p>orange fruit</p>
<pre><code>{ orange code block }
</code></pre>
<blockquote>
<p>orange block quote</p>
</blockquote>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_7() {
    let original = r##"term

:   1. Para one

       Para two
"##;
    let expected = r##"<dl>
<dt>term</dt>
<dd>
<ol>
<li><p>Para one</p>
<p>Para two</p></li>
</ol>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_8() {
    let original = r##"apple
:   red fruit
:   computer company

orange
:   orange fruit
:   telecom company
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>red fruit</dd>
<dd>computer company</dd>
<dt>orange</dt>
<dd>orange fruit</dd>
<dd>telecom company</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_9() {
    let original = r##"apple

:   red fruit

:   computer company

orange

:   orange fruit
:   telecom company
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>
<p>red fruit</p>
</dd>
<dd>
<p>computer company</p>
</dd>
<dt>orange</dt>
<dd>
<p>orange fruit</p>
</dd>
<dd>
<p>telecom company</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_10() {
    let original = r##"apple

:   red fruit

:   computer
company

orange

:   orange
fruit
:   telecom company
"##;
    let expected = r##"<dl>
<dt>apple</dt>
<dd>
<p>red fruit</p>
</dd>
<dd>
<p>computer
company</p>
</dd>
<dt>orange</dt>
<dd>
<p>orange
fruit</p>
</dd>
<dd>
<p>telecom company</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_11() {
    let original = r##"a
b\
c

:   foo
"##;
    let expected = r##"<dl>
<dt>a
b<br />
c</dt>
<dd>
<p>foo</p>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_12() {
    let original = r##"Foo

bar
:   baz

bim
:   bor
"##;
    let expected = r##"<p>Foo</p>
<dl>
<dt>bar</dt>
<dd>baz</dd>
<dt>bim</dt>
<dd>bor</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_13() {
    let original = r##"bar
:   baz

bim
:   bor

Bloze
"##;
    let expected = r##"<dl>
<dt>bar</dt>
<dd>baz</dd>
<dt>bim</dt>
<dd>bor</dd>
</dl>
<p>Bloze</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_14() {
    let original = r##"bar
    :baz

Bloze
"##;
    let expected = r##"<p>bar
:baz</p>
<p>Bloze</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_15() {
    let original = r##"bar
:    baz

Bloze
"##;
    let expected = r##"<dl>
<dt>bar</dt>
<dd>baz</dd>
</dl>
<p>Bloze</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_16() {
    let original = r##"bar
:       baz

bar
 :      baz

bar
  :     baz

bar
   :    baz

bar
    :   baz
"##;
    let expected = r##"<dl>
<dt>bar</dt>
<dd>
<pre><code>  baz
</code></pre>
</dd>
<dt>bar</dt>
<dd>
<pre><code> baz
</code></pre>
</dd>
<dt>bar</dt>
<dd>
<pre><code>baz
</code></pre>
</dd>
<dt>bar</dt>
<dd>baz</dd>
</dl>
<p>bar
:   baz</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_17() {
    let original = r##"*orange*

:   orange fruit

        { orange code block }

  > orange block quote

*orange*

:     orange fruit

        { orange code block }

  > orange block quote
"##;
    let expected = r##"<dl>
<dt><em>orange</em></dt>
<dd>
<p>orange fruit</p>
<pre><code>{ orange code block }
</code></pre>
</dd>
</dl>
<blockquote>
<p>orange block quote</p>
</blockquote>
<dl>
<dt><em>orange</em></dt>
<dd>
<pre><code>orange fruit

  { orange code block }
</code></pre>
<blockquote>
<p>orange block quote</p>
</blockquote>
</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_18() {
    let original = r##"Test|Table
----|-----
: first
"##;
    let expected = r##"<table><thead><tr><th>Test</th><th>Table</th></tr></thead><tbody></tbody>
</table>
<p>: first</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_19() {
    let original = r##"first
: second

Test|Table
----|-----
: fourth
"##;
    let expected = r##"<dl>
<dt>first</dt>
<dd>second</dd>
</dl>
<table><thead><tr><th>Test</th><th>Table</th></tr></thead><tbody></tbody>
</table>
<p>: fourth</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_20() {
    let original = r##"My section
==========
: first
"##;
    let expected = r##"<h1>My section</h1>
<p>: first</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_21() {
    let original = r##"first
: second

My section
==========
: fourth
"##;
    let expected = r##"<dl>
<dt>first</dt>
<dd>second</dd>
</dl>
<h1>My section</h1>
<p>: fourth</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_22() {
    let original = r##"## My subsection
: first
"##;
    let expected = r##"<h2>My subsection</h2>
<p>: first</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_23() {
    let original = r##"first
: second

## My subsection
: fourth
"##;
    let expected = r##"<dl>
<dt>first</dt>
<dd>second</dd>
</dl>
<h2>My subsection</h2>
<p>: fourth</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_24() {
    let original = r##"first\
: second

third  
: fourth
"##;
    let expected = r##"<dl>
<dt>first\</dt>
<dd>second</dd>
<dt>third</dt>
<dd>fourth</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_25() {
    let original = r##"<div>first</div>
: second

first
: second
<div>third</div>
: fourth
"##;
    let expected = r##"<div>first</div>
: second
<dl>
<dt>first</dt>
<dd>second</dd>
</dl>
<div>third</div>
: fourth
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_26() {
    let original = r##"<span>first</span>
: second

third
: fourth

<span>fifth</span>
: sixth
"##;
    let expected = r##"<dl>
<dt><span>first</span></dt>
<dd>second</dd>
<dt>third</dt>
<dd>fourth</dd>
<dt><span>fifth</span></dt>
<dd>sixth</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn definition_lists_test_27() {
    let original = r##"level one
: l1
    level two
    : l2
        level three
        : l3

level one
: l1
"##;
    let expected = r##"<dl>
<dt>level one</dt>
<dd>
<dl>
<dt>l1
level two</dt>
<dd>
<dl>
<dt>l2
level three</dt>
<dd>l3</dd>
</dl>
</dd>
</dl>
</dd>
<dt>level one</dt>
<dd>l1</dd>
</dl>
"##;

    test_markdown_html(original, expected, false, false, false);
}
