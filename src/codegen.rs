#[macro_export]
macro_rules! celery_app {
    (
        $name:ident,
        $broker_type:ty,
        broker_url = $broker_url:expr,
        default_queue = $default_queue:expr,
        tasks = [ $( $t:ty ),* ],
    ) => {
        use celery::{Celery, Broker, BrokerBuilder};

        lazy_static! {
            static ref $name: Celery<$broker_type> = {
                let broker_url = $broker_url;
                let default_queue = $default_queue;

                let celery: Celery<$broker_type> = Celery::builder::<<$broker_type as Broker>::Builder>("celery", &broker_url)
                    .default_queue_name(default_queue)
                    .build()
                    .unwrap();

                $(
                    celery.register_task::<$t>().unwrap();
                )*

                celery
            };
        }
    };
}
