#include "script_component.hpp"

ADDON = false;

#include "XEH_PREP.hpp"

#undef PREP
#define PREP(x) publicVariable #x
#include "XEH_PREP.hpp"

ADDON = true;