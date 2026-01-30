#include "rust/cxx.h"
#include "opencascade-sys/include/wrappers/Standard.hxx"
#include "opencascade-sys/include/wrappers/gp.hxx"
#include "opencascade-sys/include/wrappers/GeomAbs.hxx"
#include "opencascade-sys/include/wrappers/TopAbs.hxx"
#include "opencascade-sys/include/wrappers/TopLoc.hxx"
#include "opencascade-sys/include/wrappers/TColgp.hxx"
#include "opencascade-sys/include/wrappers/NCollection.hxx"
#include "opencascade-sys/include/wrappers/TopTools.hxx"
#include "opencascade-sys/include/wrappers/Geom.hxx"
#include "opencascade-sys/include/wrappers/Geom2d.hxx"
#include "opencascade-sys/include/wrappers/Poly.hxx"
#include "opencascade-sys/include/wrappers/TopoDS.hxx"
#include "opencascade-sys/include/wrappers/TopExp.hxx"
#include "opencascade-sys/include/wrappers/GProp.hxx"
#include "opencascade-sys/include/wrappers/Bnd.hxx"
#include "opencascade-sys/include/wrappers/Law.hxx"
#include "opencascade-sys/include/wrappers/BRepLib.hxx"
#include "opencascade-sys/include/wrappers/BRepAdaptor.hxx"
#include "opencascade-sys/include/wrappers/BRepBndLib.hxx"
#include "opencascade-sys/include/wrappers/BRepBuilderAPI.hxx"
#include "opencascade-sys/include/wrappers/BRepPrimAPI.hxx"
#include "opencascade-sys/include/wrappers/BRepAlgoAPI.hxx"
#include "opencascade-sys/include/wrappers/BRepFeat.hxx"
#include "opencascade-sys/include/wrappers/BRepFilletAPI.hxx"
#include "opencascade-sys/include/wrappers/BRepOffsetAPI.hxx"
#include "opencascade-sys/include/wrappers/BRepGProp.hxx"
#include "opencascade-sys/include/wrappers/BRepIntCurveSurface.hxx"
#include "opencascade-sys/include/wrappers/BRepMesh.hxx"
#include "opencascade-sys/include/wrappers/BRepTools.hxx"
#include "opencascade-sys/include/wrappers/GC.hxx"
#include "opencascade-sys/include/wrappers/GCE2d.hxx"
#include "opencascade-sys/include/wrappers/GCPnts.hxx"
#include "opencascade-sys/include/wrappers/GeomAPI.hxx"
#include "opencascade-sys/include/wrappers/IGESControl.hxx"
#include "opencascade-sys/include/wrappers/STEPControl.hxx"
#include "opencascade-sys/include/wrappers/StlAPI.hxx"
#include "opencascade-sys/include/wrappers/ShapeAnalysis.hxx"
#include "opencascade-sys/include/wrappers/ShapeUpgrade.hxx"
#include "opencascade-sys/include/wrappers/BOPAlgo.hxx"

// Generic template constructor
template <typename T, typename... Args> std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}

// Handle stuff
template <typename T> const T &handle_try_deref(const opencascade::handle<T> &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}
