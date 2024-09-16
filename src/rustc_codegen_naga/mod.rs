use rustc_codegen_ssa::traits::{CodegenBackend, ExtraBackendMethods, ModuleBufferMethods, ThinBufferMethods, WriteBackendMethods};

struct NagaModuleBuffer(Vec<u32>);


impl ModuleBufferMethods for NagaModuleBuffer {
    fn data(&self) -> &[u8] {
        todo!()
    }
}

struct NagaThinBuffer(Vec<u32>);

impl ThinBufferMethods for NagaThinBuffer {
    fn data(&self) -> &[u8] {
        todo!()
    }

    fn thin_link_data(&self) -> &[u8] {
        todo!()
    }
}


#[derive(Clone)]
struct MyBackend;

impl WriteBackendMethods for MyBackend {
    type Module = Vec<u32>;

    type TargetMachine = ();

    type TargetMachineError = String;

    type ModuleBuffer = NagaModuleBuffer;

    type ThinData = ();

    type ThinBuffer = NagaThinBuffer;

    fn run_link(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        dcx: rustc_errors::DiagCtxtHandle<'_>,
        modules: Vec<rustc_codegen_ssa::ModuleCodegen<Self::Module>>,
    ) -> Result<rustc_codegen_ssa::ModuleCodegen<Self::Module>, rustc_errors::FatalError> {
        todo!()
    }

    fn run_fat_lto(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        modules: Vec<rustc_codegen_ssa::back::write::FatLtoInput<Self>>,
        cached_modules: Vec<(rustc_codegen_ssa::back::lto::SerializedModule<Self::ModuleBuffer>, rustc_middle::dep_graph::WorkProduct)>,
    ) -> Result<rustc_codegen_ssa::back::lto::LtoModuleCodegen<Self>, rustc_errors::FatalError> {
        todo!()
    }

    fn run_thin_lto(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        modules: Vec<(String, Self::ThinBuffer)>,
        cached_modules: Vec<(rustc_codegen_ssa::back::lto::SerializedModule<Self::ModuleBuffer>, rustc_middle::dep_graph::WorkProduct)>,
    ) -> Result<(Vec<rustc_codegen_ssa::back::lto::LtoModuleCodegen<Self>>, Vec<rustc_middle::dep_graph::WorkProduct>), rustc_errors::FatalError> {
        todo!()
    }

    fn print_pass_timings(&self) {
        todo!()
    }

    fn print_statistics(&self) {
        todo!()
    }

    unsafe fn optimize(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        dcx: rustc_errors::DiagCtxtHandle<'_>,
        module: &rustc_codegen_ssa::ModuleCodegen<Self::Module>,
        config: &rustc_codegen_ssa::back::write::ModuleConfig,
    ) -> Result<(), rustc_errors::FatalError> {
        todo!()
    }

    fn optimize_fat(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        llmod: &mut rustc_codegen_ssa::ModuleCodegen<Self::Module>,
    ) -> Result<(), rustc_errors::FatalError> {
        todo!()
    }

    unsafe fn optimize_thin(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        thin: rustc_codegen_ssa::back::lto::ThinModule<Self>,
    ) -> Result<rustc_codegen_ssa::ModuleCodegen<Self::Module>, rustc_errors::FatalError> {
        todo!()
    }

    unsafe fn codegen(
        cgcx: &rustc_codegen_ssa::back::write::CodegenContext<Self>,
        dcx: rustc_errors::DiagCtxtHandle<'_>,
        module: rustc_codegen_ssa::ModuleCodegen<Self::Module>,
        config: &rustc_codegen_ssa::back::write::ModuleConfig,
    ) -> Result<rustc_codegen_ssa::CompiledModule, rustc_errors::FatalError> {
        todo!()
    }

    fn prepare_thin(
        module: rustc_codegen_ssa::ModuleCodegen<Self::Module>,
        want_summary: bool,
    ) -> (String, Self::ThinBuffer) {
        todo!()
    }

    fn serialize_module(module: rustc_codegen_ssa::ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        todo!()
    }
}

impl ExtraBackendMethods for MyBackend {
    fn codegen_allocator<'tcx>(
        &self,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        module_name: &str,
        kind: rustc_ast::expand::allocator::AllocatorKind,
        alloc_error_handler_kind: rustc_ast::expand::allocator::AllocatorKind,
    ) -> Self::Module {
        todo!()
    }

    fn compile_codegen_unit(
        &self,
        tcx: rustc_middle::ty::TyCtxt<'_>,
        cgu_name: rustc_span::Symbol,
    ) -> (rustc_codegen_ssa::ModuleCodegen<Self::Module>, u64) {
        todo!()
    }

    fn target_machine_factory(
        &self,
        sess: &rustc_session::Session,
        opt_level: rustc_session::config::OptLevel,
        target_features: &[String],
    ) -> rustc_codegen_ssa::back::write::TargetMachineFactoryFn<Self> {
        todo!()
    }
}

impl CodegenBackend for MyBackend {
    fn locale_resource(&self) -> &'static str {
        todo!()
    }

    fn codegen_crate<'tcx>(
        &self,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        metadata: rustc_metadata::EncodedMetadata,
        need_metadata_module: bool,
    ) -> Box<dyn std::any::Any> {
        Box::new(rustc_codegen_ssa::base::codegen_crate(
            Self,
            tcx,
            tcx.sess
                .opts
                .cg
                .target_cpu
                .clone()
                .unwrap_or_else(|| tcx.sess.target.cpu.to_string()),
            metadata,
            need_metadata_module,
        ))
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn std::any::Any>,
        sess: &rustc_session::Session,
        outputs: &rustc_session::config::OutputFilenames,
    ) -> (rustc_codegen_ssa::CodegenResults, rustc_data_structures::fx::FxIndexMap<rustc_middle::dep_graph::WorkProductId, rustc_middle::dep_graph::WorkProduct>) {
        todo!()
    }

    fn link(
        &self,
        sess: &rustc_session::Session,
        codegen_results: rustc_codegen_ssa::CodegenResults,
        outputs: &rustc_session::config::OutputFilenames,
    ) -> Result<(), rustc_span::ErrorGuaranteed> {
        todo!()
    }
   // Implement codegen methods
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(MyBackend)
}