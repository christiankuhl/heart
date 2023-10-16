def vol(fn, space=False):
    if space:
        return len([c for l in open(fn).readlines()[2:-2] for c in l.strip() if c != " "])
    else:
        return len([c for l in open(fn).readlines() for c in l.strip()])

print(vol("src/main.rs"))
print(vol("template.txt", True))