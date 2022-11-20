BEGIN {
    in_member=0
}

{
    if (/^export/) {
        in_doc_block=0
        if (match($0, /}$/)) {
            in_member = 1
        }
        match($0, /(function|type) (\w+)/, m)

        if (m[2] == "InitInput") {
            # discard this type as we are not using it
        } else if (m[2] != "init" && substr(m[2], 1, 2) != "__") {
            # Skip export of init and private methods
            print docblock_buf
            print $0

            if (match($0, /export (function|class) (\w+)/, m)) {
                public_members = public_members "\n\n" \
                    gensub(/(^|\n)/, "\\1    ", "g", docblock_buf) "\n" \
                    "    readonly " m[2] ": typeof " m[2] ";"
            }
        }
        docblock_buf = ""
    } else if ($0 == "/**") {
        in_doc_block = 1
        docblock_buf = $0 "\n"
    } else if (in_doc_block && $0 == "*/") {
        in_doc_block = 0
        if (in_member) {
            print docblock_buf
            print "  " $0

            docblock_buf = ""
        } else {
            docblock_buf = docblock_buf " " $0
        }
    } else if (in_doc_block && substr($0, 1, 1) == "*") {
        if (in_member) docblock_buf = docblock_buf "  "
        docblock_buf = docblock_buf " " $0 "\n"
    } else {
        print
    }
}

END {
    print "export interface QMCCryptoInstance {"
    print "    readonly _instance: InitOutput;"
    print substr(public_members, 2)
    print "}"

    print "/**"
    print " * Initialise and enable other public methods (from first instance)."
    print " * @returns {Promise<InitOutput>}"
    print " */"
    print "export default function init (): Promise<QMCCryptoInstance>;"
    print ""
}