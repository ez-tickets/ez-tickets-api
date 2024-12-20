use nitinol::projection::Projector;

pub trait DependOnEventProjector: 'static + Sync + Send {
    fn projector(&self) -> &Projector;
}