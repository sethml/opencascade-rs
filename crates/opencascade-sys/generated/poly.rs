#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_poly.hxx");
        #[doc = "Pnt from gp module"]
        type gp_Pnt = crate::gp::ffi::Pnt;
        #[doc = "Pnt2d from gp module"]
        type gp_Pnt2d = crate::gp::ffi::Pnt2d;
        #[doc = "Vec from gp module"]
        type gp_Vec = crate::gp::ffi::Vec_;
        #[doc = "Vec2d from gp module"]
        type gp_Vec2d = crate::gp::ffi::Vec2d;
        #[doc = "Dir from gp module"]
        type gp_Dir = crate::gp::ffi::Dir;
        #[doc = "Dir2d from gp module"]
        type gp_Dir2d = crate::gp::ffi::Dir2d;
        #[doc = "XYZ from gp module"]
        type gp_XYZ = crate::gp::ffi::XYZ;
        #[doc = "Ax1 from gp module"]
        type gp_Ax1 = crate::gp::ffi::Ax1;
        #[doc = "Ax2 from gp module"]
        type gp_Ax2 = crate::gp::ffi::Ax2;
        #[doc = "Ax2d from gp module"]
        type gp_Ax2d = crate::gp::ffi::Ax2d;
        #[doc = "Ax3 from gp module"]
        type gp_Ax3 = crate::gp::ffi::Ax3;
        #[doc = "Trsf from gp module"]
        type gp_Trsf = crate::gp::ffi::Trsf;
        #[doc = "Trsf2d from gp module"]
        type gp_Trsf2d = crate::gp::ffi::Trsf2d;
        #[doc = "GTrsf from gp module"]
        type gp_GTrsf = crate::gp::ffi::GTrsf;
        #[doc = "GTrsf2d from gp module"]
        type gp_GTrsf2d = crate::gp::ffi::GTrsf2d;
        #[doc = "Lin from gp module"]
        type gp_Lin = crate::gp::ffi::Lin;
        #[doc = "Circ from gp module"]
        type gp_Circ = crate::gp::ffi::Circ;
        #[doc = "Pln from gp module"]
        type gp_Pln = crate::gp::ffi::Pln;
        #[doc = "Box from bnd module"]
        type Bnd_Box = crate::bnd::ffi::Box_;
        #[doc = "OBB from bnd module"]
        type Bnd_OBB = crate::bnd::ffi::OBB;
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_HArray1OfPnt2d"]
        type TColgp_HArray1OfPnt2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt"]
        type TColgp_Array1OfPnt;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_ArrayOfUVNodes"]
        type Poly_ArrayOfUVNodes;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TShort_HArray1OfShortReal"]
        type TShort_HArray1OfShortReal;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_ArrayOfNodes"]
        type Poly_ArrayOfNodes;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_TriangulationParameters"]
        type Poly_TriangulationParameters;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_Array1OfTriangle"]
        type Poly_Array1OfTriangle;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt2d"]
        type TColgp_Array1OfPnt2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Vec3f"]
        type gp_Vec3f;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_MeshPurpose"]
        type Poly_MeshPurpose;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "OSD_FileSystem"]
        type OSD_FileSystem;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_Triangle"]
        type Poly_Triangle;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_HArray1OfTriangle"]
        type Poly_HArray1OfTriangle;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyTriangulation"]
        type HandlePolyTriangulation;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTColgpHArray1OfPnt2d"]
        type HandleTColgpHArray1OfPnt2d;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTColgpHArray1OfPnt"]
        type HandleTColgpHArray1OfPnt;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleOSDFileSystem"]
        type HandleOSDFileSystem;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyTriangulationParameters"]
        type HandlePolyTriangulationParameters;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyHArray1OfTriangle"]
        type HandlePolyHArray1OfTriangle;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTShortHArray1OfShortReal"]
        type HandleTShortHArray1OfShortReal;
        #[doc = " ======================== Poly_Triangulation ========================"]
        #[doc = "Provides a triangulation for a surface, a set of surfaces, or more generally a shape. A triangulation consists of an approximate representation of the actual shape, using a collection of points and triangles. The points are located on the surface. The edges of the triangles connect adjacent points with a straight line that approximates the true curve on the surface. A triangulation comprises: - A table of 3D nodes (3D points on the surface). - A table of triangles. Each triangle (Poly_Triangle object) comprises a triplet of indices in the table of 3D nodes specific to the triangulation. - An optional table of 2D nodes (2D points), parallel to the table of 3D nodes. 2D point are the (u, v) parameters of the corresponding 3D point on the surface approximated by the triangulation. - An optional table of 3D vectors, parallel to the table of 3D nodes, defining normals to the surface at specified 3D point. - An optional deflection, which maximizes the distance from a point on the surface to the corresponding point on its approximate triangulation. In many cases, algorithms do not need to work with the exact representation of a surface. A triangular representation induces simpler and more robust adjusting, faster performances, and the results are as good."]
        #[cxx_name = "Poly_Triangulation"]
        type Triangulation;
        #[doc = "Constructs an empty triangulation."]
        #[cxx_name = "Poly_Triangulation_ctor"]
        fn Triangulation_ctor() -> UniquePtr<Triangulation>;
        #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized without a triangle or a node, but capable of containing specified number of nodes and triangles. @param[in] theNbNodes      number of nodes to allocate @param[in] theNbTriangles  number of triangles to allocate @param[in] theHasUVNodes   indicates whether 2D nodes will be associated with 3D ones, (i.e. to enable a 2D representation) @param[in] theHasNormals   indicates whether normals will be given and associated with nodes"]
        #[cxx_name = "Poly_Triangulation_ctor_int2_bool2"]
        fn Triangulation_ctor_int2_bool2(
            theNbNodes: i32,
            theNbTriangles: i32,
            theHasUVNodes: bool,
            theHasNormals: bool,
        ) -> UniquePtr<Triangulation>;
        #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized with 3D points from Nodes and triangles from Triangles."]
        #[cxx_name = "Poly_Triangulation_ctor_array1ofpnt_array1oftriangle"]
        fn Triangulation_ctor_array1ofpnt_array1oftriangle(
            Nodes: &TColgp_Array1OfPnt,
            Triangles: &Poly_Array1OfTriangle,
        ) -> UniquePtr<Triangulation>;
        #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized with 3D points from Nodes, 2D points from UVNodes and triangles from Triangles, where coordinates of a 2D point from UVNodes are the (u, v) parameters of the corresponding 3D point from Nodes on the surface approximated by the constructed triangulation."]
        #[cxx_name = "Poly_Triangulation_ctor_array1ofpnt_array1ofpnt2d_array1oftriangle"]
        fn Triangulation_ctor_array1ofpnt_array1ofpnt2d_array1oftriangle(
            Nodes: &TColgp_Array1OfPnt,
            UVNodes: &TColgp_Array1OfPnt2d,
            Triangles: &Poly_Array1OfTriangle,
        ) -> UniquePtr<Triangulation>;
        #[doc = "Copy constructor for triangulation."]
        #[cxx_name = "Poly_Triangulation_ctor_handletriangulation"]
        fn Triangulation_ctor_handletriangulation(
            theTriangulation: &HandlePolyTriangulation,
        ) -> UniquePtr<Triangulation>;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Triangulation) -> &HandleStandardType;
        #[doc = "Returns the deflection of this triangulation."]
        #[cxx_name = "Deflection"]
        fn deflection(self: &Triangulation) -> f64;
        #[doc = "Sets the deflection of this triangulation to theDeflection. See more on deflection in Polygon2D"]
        #[cxx_name = "Deflection"]
        fn deflectionreal_2(self: Pin<&mut Triangulation>, theDeflection: f64);
        #[doc = "Returns initial set of parameters used to generate this triangulation."]
        #[cxx_name = "Parameters"]
        fn parameters(self: &Triangulation) -> &HandlePolyTriangulationParameters;
        #[doc = "Updates initial set of parameters used to generate this triangulation."]
        #[cxx_name = "Parameters"]
        fn parametershandletriangulationparameters_2(
            self: Pin<&mut Triangulation>,
            theParams: &HandlePolyTriangulationParameters,
        );
        #[doc = "Clears internal arrays of nodes and all attributes."]
        #[cxx_name = "Clear"]
        fn clear(self: Pin<&mut Triangulation>);
        #[doc = "Returns TRUE if triangulation has some geometry."]
        #[cxx_name = "HasGeometry"]
        fn has_geometry(self: &Triangulation) -> bool;
        #[doc = "Returns the number of nodes for this triangulation."]
        #[cxx_name = "NbNodes"]
        fn nb_nodes(self: &Triangulation) -> i32;
        #[doc = "Returns the number of triangles for this triangulation."]
        #[cxx_name = "NbTriangles"]
        fn nb_triangles(self: &Triangulation) -> i32;
        #[doc = "Returns Standard_True if 2D nodes are associated with 3D nodes for this triangulation."]
        #[cxx_name = "HasUVNodes"]
        fn has_uv_nodes(self: &Triangulation) -> bool;
        #[doc = "Returns Standard_True if nodal normals are defined."]
        #[cxx_name = "HasNormals"]
        fn has_normals(self: &Triangulation) -> bool;
        #[doc = "Sets a node coordinates. @param[in] theIndex node index within [1, NbNodes()] range @param[in] thePnt   3D point coordinates"]
        #[cxx_name = "SetNode"]
        fn set_node(self: Pin<&mut Triangulation>, theIndex: i32, thePnt: &gp_Pnt);
        #[doc = "Sets an UV-node coordinates. @param[in] theIndex node index within [1, NbNodes()] range @param[in] thePnt   UV coordinates"]
        #[cxx_name = "SetUVNode"]
        fn set_uv_node(self: Pin<&mut Triangulation>, theIndex: i32, thePnt: &gp_Pnt2d);
        #[doc = "Returns triangle at the given index. @param[in] theIndex triangle index within [1, NbTriangles()] range @return triangle node indices, with each node defined within [1, NbNodes()] range"]
        #[cxx_name = "Triangle"]
        fn triangle(self: &Triangulation, theIndex: i32) -> &Poly_Triangle;
        #[doc = "Sets a triangle. @param[in] theIndex triangle index within [1, NbTriangles()] range @param[in] theTriangle triangle node indices, with each node defined within [1, NbNodes()] range"]
        #[cxx_name = "SetTriangle"]
        fn set_triangle(self: Pin<&mut Triangulation>, theIndex: i32, theTriangle: &Poly_Triangle);
        #[doc = "Returns normal at the given index. @param[in]  theIndex node index within [1, NbNodes()] range @param[out] theVec3  3D vector defining a surface normal"]
        #[cxx_name = "Normal"]
        fn normalint(self: &Triangulation, theIndex: i32, theVec3: Pin<&mut gp_Vec3f>);
        #[doc = "Changes normal at the given index. @param[in] theIndex node index within [1, NbNodes()] range @param[in] theVec3  normalized 3D vector defining a surface normal"]
        #[cxx_name = "SetNormal"]
        fn set_normalint(self: Pin<&mut Triangulation>, theIndex: i32, theNormal: &gp_Vec3f);
        #[doc = "Changes normal at the given index. @param[in] theIndex  node index within [1, NbNodes()] range @param[in] theNormal normalized 3D vector defining a surface normal"]
        #[cxx_name = "SetNormal"]
        fn set_normalint_2(self: Pin<&mut Triangulation>, theIndex: i32, theNormal: &gp_Dir);
        #[doc = "Returns mesh purpose bits."]
        #[cxx_name = "MeshPurpose"]
        fn mesh_purpose(self: &Triangulation) -> u32;
        #[doc = "Returns cached min - max range of triangulation data, which is VOID by default (e.g, no cached information)."]
        #[cxx_name = "CachedMinMax"]
        fn cached_min_max(self: &Triangulation) -> &Bnd_Box;
        #[doc = "Sets a cached min - max range of this triangulation. The bounding box should exactly match actual range of triangulation data without a gap or transformation, or otherwise undefined behavior will be observed. Passing a VOID range invalidates the cache."]
        #[cxx_name = "SetCachedMinMax"]
        fn set_cached_min_max(self: Pin<&mut Triangulation>, theBox: &Bnd_Box);
        #[doc = "Returns TRUE if there is some cached min - max range of this triangulation."]
        #[cxx_name = "HasCachedMinMax"]
        fn has_cached_min_max(self: &Triangulation) -> bool;
        #[doc = "Updates cached min - max range of this triangulation with bounding box of nodal data."]
        #[cxx_name = "UpdateCachedMinMax"]
        fn update_cached_min_max(self: Pin<&mut Triangulation>);
        #[doc = "Returns TRUE if node positions are defined with double precision; TRUE by default."]
        #[cxx_name = "IsDoublePrecision"]
        fn is_double_precision(self: &Triangulation) -> bool;
        #[doc = "Set if node positions should be defined with double or single precision for 3D and UV nodes. Raises exception if data was already allocated."]
        #[cxx_name = "SetDoublePrecision"]
        fn set_double_precision(self: Pin<&mut Triangulation>, theIsDouble: bool);
        #[doc = "Method resizing internal arrays of nodes (synchronously for all attributes). @param[in] theNbNodes    new number of nodes @param[in] theToCopyOld  copy old nodes into the new array"]
        #[cxx_name = "ResizeNodes"]
        fn resize_nodes(self: Pin<&mut Triangulation>, theNbNodes: i32, theToCopyOld: bool);
        #[doc = "Method resizing an internal array of triangles. @param[in] theNbTriangles  new number of triangles @param[in] theToCopyOld    copy old triangles into the new array"]
        #[cxx_name = "ResizeTriangles"]
        fn resize_triangles(self: Pin<&mut Triangulation>, theNbTriangles: i32, theToCopyOld: bool);
        #[doc = "If an array for UV coordinates is not allocated yet, do it now."]
        #[cxx_name = "AddUVNodes"]
        fn add_uv_nodes(self: Pin<&mut Triangulation>);
        #[doc = "Deallocates the UV nodes array."]
        #[cxx_name = "RemoveUVNodes"]
        fn remove_uv_nodes(self: Pin<&mut Triangulation>);
        #[doc = "If an array for normals is not allocated yet, do it now."]
        #[cxx_name = "AddNormals"]
        fn add_normals(self: Pin<&mut Triangulation>);
        #[doc = "Deallocates the normals array."]
        #[cxx_name = "RemoveNormals"]
        fn remove_normals(self: Pin<&mut Triangulation>);
        #[doc = "Compute smooth normals by averaging triangle normals."]
        #[cxx_name = "ComputeNormals"]
        fn compute_normals(self: Pin<&mut Triangulation>);
        #[doc = "Returns an internal array of triangles. Triangle()/SetTriangle() should be used instead in portable code."]
        #[cxx_name = "InternalTriangles"]
        fn internal_triangles(self: Pin<&mut Triangulation>) -> Pin<&mut Poly_Array1OfTriangle>;
        #[doc = "Returns an internal array of nodes. Node()/SetNode() should be used instead in portable code."]
        #[cxx_name = "InternalNodes"]
        fn internal_nodes(self: Pin<&mut Triangulation>) -> Pin<&mut Poly_ArrayOfNodes>;
        #[doc = "Returns an internal array of UV nodes. UBNode()/SetUVNode() should be used instead in portable code."]
        #[cxx_name = "InternalUVNodes"]
        fn internal_uv_nodes(self: Pin<&mut Triangulation>) -> Pin<&mut Poly_ArrayOfUVNodes>;
        #[cxx_name = "SetNormals"]
        fn set_normals(self: Pin<&mut Triangulation>, theNormals: &HandleTShortHArray1OfShortReal);
        #[cxx_name = "Triangles"]
        fn triangles(self: &Triangulation) -> &Poly_Array1OfTriangle;
        #[cxx_name = "ChangeTriangles"]
        fn change_triangles(self: Pin<&mut Triangulation>) -> Pin<&mut Poly_Array1OfTriangle>;
        #[cxx_name = "ChangeTriangle"]
        fn change_triangle(self: Pin<&mut Triangulation>, theIndex: i32)
            -> Pin<&mut Poly_Triangle>;
        #[doc = "@name late-load deferred data interface Returns number of deferred nodes that can be loaded using LoadDeferredData(). Note: this is estimated values, which might be different from actually loaded values. Always check triangulation size of actually loaded data in code to avoid out-of-range issues."]
        #[cxx_name = "NbDeferredNodes"]
        fn nb_deferred_nodes(self: &Triangulation) -> i32;
        #[doc = "Returns number of deferred triangles that can be loaded using LoadDeferredData(). Note: this is estimated values, which might be different from actually loaded values Always check triangulation size of actually loaded data in code to avoid out-of-range issues."]
        #[cxx_name = "NbDeferredTriangles"]
        fn nb_deferred_triangles(self: &Triangulation) -> i32;
        #[doc = "Returns TRUE if there is some triangulation data that can be loaded using LoadDeferredData()."]
        #[cxx_name = "HasDeferredData"]
        fn has_deferred_data(self: &Triangulation) -> bool;
        #[doc = "Loads triangulation data into itself from some deferred storage using specified shared input file system."]
        #[cxx_name = "LoadDeferredData"]
        fn load_deferred_data(
            self: Pin<&mut Triangulation>,
            theFileSystem: &HandleOSDFileSystem,
        ) -> bool;
        #[doc = "Releases triangulation data if it has connected deferred storage."]
        #[cxx_name = "UnloadDeferredData"]
        fn unload_deferred_data(self: Pin<&mut Triangulation>) -> bool;
        #[doc = "Creates full copy of current triangulation"]
        #[cxx_name = "Poly_Triangulation_Copy"]
        fn Triangulation_copy(self_: &Triangulation) -> UniquePtr<HandlePolyTriangulation>;
        #[doc = "Returns a node at the given index. @param[in] theIndex node index within [1, NbNodes()] range @return 3D point coordinates"]
        #[cxx_name = "Poly_Triangulation_Node"]
        fn Triangulation_node(self_: &Triangulation, theIndex: i32) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns UV-node at the given index. @param[in] theIndex node index within [1, NbNodes()] range @return 2D point defining UV coordinates"]
        #[cxx_name = "Poly_Triangulation_UVNode"]
        fn Triangulation_uv_node(self_: &Triangulation, theIndex: i32) -> UniquePtr<gp_Pnt2d>;
        #[doc = "Returns normal at the given index. @param[in] theIndex node index within [1, NbNodes()] range @return normalized 3D vector defining a surface normal"]
        #[cxx_name = "Poly_Triangulation_Normal"]
        fn Triangulation_normal(self_: &Triangulation, theIndex: i32) -> UniquePtr<gp_Dir>;
        #[doc = "Returns the table of 3D points for read-only access or NULL if nodes array is undefined. Poly_Triangulation::Node() should be used instead when possible. Returned object should not be used after Poly_Triangulation destruction."]
        #[cxx_name = "Poly_Triangulation_MapNodeArray"]
        fn Triangulation_map_node_array(
            self_: &Triangulation,
        ) -> UniquePtr<HandleTColgpHArray1OfPnt>;
        #[doc = "Returns the triangle array for read-only access or NULL if triangle array is undefined. Poly_Triangulation::Triangle() should be used instead when possible. Returned object should not be used after Poly_Triangulation destruction."]
        #[cxx_name = "Poly_Triangulation_MapTriangleArray"]
        fn Triangulation_map_triangle_array(
            self_: &Triangulation,
        ) -> UniquePtr<HandlePolyHArray1OfTriangle>;
        #[doc = "Returns the table of 2D nodes for read-only access or NULL if UV nodes array is undefined. Poly_Triangulation::UVNode() should be used instead when possible. Returned object should not be used after Poly_Triangulation destruction."]
        #[cxx_name = "Poly_Triangulation_MapUVNodeArray"]
        fn Triangulation_map_uv_node_array(
            self_: &Triangulation,
        ) -> UniquePtr<HandleTColgpHArray1OfPnt2d>;
        #[doc = "Returns the table of per-vertex normals for read-only access or NULL if normals array is undefined. Poly_Triangulation::Normal() should be used instead when possible. Returned object should not be used after Poly_Triangulation destruction."]
        #[cxx_name = "Poly_Triangulation_MapNormalArray"]
        fn Triangulation_map_normal_array(
            self_: &Triangulation,
        ) -> UniquePtr<HandleTShortHArray1OfShortReal>;
        #[doc = "Loads triangulation data into new Poly_Triangulation object from some deferred storage using specified shared input file system."]
        #[cxx_name = "Poly_Triangulation_DetachedLoadDeferredData"]
        fn Triangulation_detached_load_deferred_data(
            self_: &Triangulation,
            theFileSystem: &HandleOSDFileSystem,
        ) -> UniquePtr<HandlePolyTriangulation>;
        #[cxx_name = "Poly_Triangulation_get_type_name"]
        fn Triangulation_get_type_name() -> String;
    }
    impl UniquePtr<Triangulation> {}
}
pub use ffi::Triangulation;
impl Triangulation {
    #[doc = "Constructs an empty triangulation."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Triangulation_ctor()
    }

    #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized without a triangle or a node, but capable of containing specified number of nodes and triangles. @param[in] theNbNodes      number of nodes to allocate @param[in] theNbTriangles  number of triangles to allocate @param[in] theHasUVNodes   indicates whether 2D nodes will be associated with 3D ones, (i.e. to enable a 2D representation) @param[in] theHasNormals   indicates whether normals will be given and associated with nodes"]
    pub fn new_int2_bool2(
        theNbNodes: i32,
        theNbTriangles: i32,
        theHasUVNodes: bool,
        theHasNormals: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Triangulation_ctor_int2_bool2(theNbNodes, theNbTriangles, theHasUVNodes, theHasNormals)
    }

    #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized with 3D points from Nodes and triangles from Triangles."]
    pub fn new_array1ofpnt_array1oftriangle(
        Nodes: &ffi::TColgp_Array1OfPnt,
        Triangles: &ffi::Poly_Array1OfTriangle,
    ) -> cxx::UniquePtr<Self> {
        ffi::Triangulation_ctor_array1ofpnt_array1oftriangle(Nodes, Triangles)
    }

    #[doc = "Constructs a triangulation from a set of triangles. The triangulation is initialized with 3D points from Nodes, 2D points from UVNodes and triangles from Triangles, where coordinates of a 2D point from UVNodes are the (u, v) parameters of the corresponding 3D point from Nodes on the surface approximated by the constructed triangulation."]
    pub fn new_array1ofpnt_array1ofpnt2d_array1oftriangle(
        Nodes: &ffi::TColgp_Array1OfPnt,
        UVNodes: &ffi::TColgp_Array1OfPnt2d,
        Triangles: &ffi::Poly_Array1OfTriangle,
    ) -> cxx::UniquePtr<Self> {
        ffi::Triangulation_ctor_array1ofpnt_array1ofpnt2d_array1oftriangle(
            Nodes, UVNodes, Triangles,
        )
    }

    #[doc = "Copy constructor for triangulation."]
    pub fn new_handletriangulation(
        theTriangulation: &ffi::HandlePolyTriangulation,
    ) -> cxx::UniquePtr<Self> {
        ffi::Triangulation_ctor_handletriangulation(theTriangulation)
    }
}
