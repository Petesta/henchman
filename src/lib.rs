pub fn application_job_template() -> String {
    "class ApplicationJob < ActiveJob::Base\nend".to_string()
}

pub fn job_template<A: AsRef<str>>(class_name: A, queue_name: A) -> String {
    format!(
        "
        class {} < ApplicationJob
          queue_as {}\n
          def perform(*args)
            # Do something later
          end
        end
        ",
        class_name.as_ref(),
        queue_name.as_ref()
    )
}

fn main() {
    println!("{}", application_job_template());
    println!("{}", job_template("VL", "Worker"));
}
