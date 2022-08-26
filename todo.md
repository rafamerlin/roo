- [ ] Have a command runner class (for each different OS as well)
- [ ] Have a selector class and a generic implementation for the selector
- [ ] Have a git class
- [ ] Have a opener class?
- [ ] Improve readme with a gif of it working


Pseudo Main:

fn main() {
    let configs = get_configs();
    let request = get_inputs();

    let (operation, selector) = get_operation(configs, request);

    let selected = request_selection(selector);

    run_operation(operation, selected);
}