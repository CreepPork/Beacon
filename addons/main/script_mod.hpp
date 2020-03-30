#define MAINPREFIX x
#define PREFIX beacon

#include "script_version.hpp"

#define VERSION     MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCHLVL.BUILD
#define VERSION_AR  MAJOR,MINOR,PATCHLVL,BUILD

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 1.96
#define REQUIRED_CBA_VERSION {3,14,0}

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(BEACON - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(BEACON - COMPONENT)
#endif

#define EXTENSION_NAME beacon
