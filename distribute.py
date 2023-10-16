code = "".join(l.strip() for l in open("src/_main.rs").readlines())
template = "".join(l for l in open("template.txt").readlines())

src = ""
while len(code)>0 and len(template)>0:
    t, template = template[0], template[1:]
    if t in " \n":
        src += t
    else:
        c, code = code[0], code[1:]
        src += c

import re
src = re.sub(r" +$", "", src, flags=re.MULTILINE)

with open("src/main.rs", "w") as f:
    f.write(src)