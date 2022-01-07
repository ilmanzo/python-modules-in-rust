import sys,os 

# for development we include the rust build path to the path
# that python looks for modules
for p in ("release","debug"):
    sys.path.append(os.path.join("target",p))

# now this can be resolved
from librusty import list_prod


result=list_prod([10,3,6])

print(result)