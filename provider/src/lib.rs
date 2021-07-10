pub mod connector;

pub trait ContainerProvider {
    fn list_containers();
}