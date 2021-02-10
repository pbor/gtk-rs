// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdalign.h>

typedef struct {
    const char *name;
    size_t size;
    size_t alignent;
} Layout;

const Layout LAYOUTS[] = {
    { "GdkColorspace", sizeof(GdkColorspace), alignof(GdkColorspace) },
    { "GdkInterpType", sizeof(GdkInterpType), alignof(GdkInterpType) },
    { "GdkPixbufAlphaMode", sizeof(GdkPixbufAlphaMode), alignof(GdkPixbufAlphaMode) },
    { "GdkPixbufAnimation", sizeof(GdkPixbufAnimation), alignof(GdkPixbufAnimation) },
    { "GdkPixbufAnimationClass", sizeof(GdkPixbufAnimationClass), alignof(GdkPixbufAnimationClass) },
    { "GdkPixbufAnimationIter", sizeof(GdkPixbufAnimationIter), alignof(GdkPixbufAnimationIter) },
    { "GdkPixbufAnimationIterClass", sizeof(GdkPixbufAnimationIterClass), alignof(GdkPixbufAnimationIterClass) },
    { "GdkPixbufError", sizeof(GdkPixbufError), alignof(GdkPixbufError) },
    { "GdkPixbufFormat", sizeof(GdkPixbufFormat), alignof(GdkPixbufFormat) },
    { "GdkPixbufFormatFlags", sizeof(GdkPixbufFormatFlags), alignof(GdkPixbufFormatFlags) },
    { "GdkPixbufLoader", sizeof(GdkPixbufLoader), alignof(GdkPixbufLoader) },
    { "GdkPixbufLoaderClass", sizeof(GdkPixbufLoaderClass), alignof(GdkPixbufLoaderClass) },
    { "GdkPixbufModule", sizeof(GdkPixbufModule), alignof(GdkPixbufModule) },
    { "GdkPixbufModulePattern", sizeof(GdkPixbufModulePattern), alignof(GdkPixbufModulePattern) },
    { "GdkPixbufRotation", sizeof(GdkPixbufRotation), alignof(GdkPixbufRotation) }
};

const Layout *c_layouts(size_t *n) {
    *n = sizeof(LAYOUTS) / sizeof(Layout);
    return LAYOUTS;
}
