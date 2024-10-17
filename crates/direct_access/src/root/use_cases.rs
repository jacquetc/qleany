mod create_uc;

trait RootRepository {
    fn create(&self, name: String) -> Result<(), String>;
}