use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dfx_base::{data_dictionary::DataDictionary, message::Message, message_factory::DefaultMessageFactory};
use pprof::criterion::{PProfProfiler, Output};

fn criterion_benchmark_data_dictionary(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing data dictionary:");
    group
        .significance_level(0.01)
        .sample_size(500)
        .measurement_time(Duration::from_secs(30));
    group.bench_function("FIXT11.xml", |b| {
        let data = include_str!("../../../spec/FIXT11.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX40.xml", |b| {
        let data = include_str!("../../../spec/FIX40.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX41.xml", |b| {
        let data = include_str!("../../../spec/FIX41.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX42.xml", |b| {
        let data = include_str!("../../../spec/FIX42.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX43.xml", |b| {
        let data = include_str!("../../../spec/FIX43.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX44.xml", |b| {
        let data = include_str!("../../../spec/FIX44.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX50.xml", |b| {
        let data = include_str!("../../../spec/FIX50.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX50SP1.xml", |b| {
        let data = include_str!("../../../spec/FIX50SP1.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.bench_function("FIX50SP2.xml", |b| {
        let data = include_str!("../../../spec/FIX50SP2.xml");
        b.iter(|| DataDictionary::load_from_string(black_box(data)))
    });
    group.finish()
}

fn criterion_benchmark_message(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing message with data dictionary");
    // group.bench_function("FIXT11.xml", |b| {
    //     let data = include_str!("../../../spec/FIXT11.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX40.xml", |b| {
    //     let data = include_str!("../../../spec/FIX40.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX41.xml", |b| {
    //     let data = include_str!("../../../spec/FIX41.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX42.xml", |b| {
    //     let data = include_str!("../../../spec/FIX42.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX43.xml", |b| {
    //     let data = include_str!("../../../spec/FIX43.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });

    let data = include_str!("../../../spec/FIX44.xml");
    let dd = DataDictionary::load_from_string(data).unwrap();
    let message = b"8=FIX.4.4\x019=115\x0135=A\x0134=1\x0149=sender-comp-id\x0152=20221025-10:49:30.969\x0156=target-comp-id\x0198=0\x01108=30\x01141=Y\x01553=username\x01554=password\x0110=159\x01";
    let mut msg = Message::default();
    let r = msg.from_string::<DefaultMessageFactory>(message, true, Some(&dd), Some(&dd), None, false);
    eprintln!("{:?}", r);
    assert!(r.is_ok());
    group.bench_function("FIX44.xml", |b| {
        b.iter(|| {
            let mut msg = Message::default();
            msg.from_string::<DefaultMessageFactory>(message, true, Some(&dd), Some(&dd), None, false)
        })
    });
    group.bench_function("NO.xml", |b| {
        b.iter(|| {
            let mut msg = Message::default();
            msg.from_string::<DefaultMessageFactory>(message, true, None, None, None, false)
        })
    });
    // group.bench_function("FIX50.xml", |b| {
    //     let data = include_str!("../../../spec/FIX50.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX50SP1.xml", |b| {
    //     let data = include_str!("../../../spec/FIX50SP1.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
    // group.bench_function("FIX50SP2.xml", |b| {
    //     let data = include_str!("../../../spec/FIX50SP2.xml");
    //     let dd = DataDictionary::load_from_string(data).unwrap();
    //     let message = [];
    //     b.iter(|| {
    //         let mut msg = Message::default();
    //         msg.from_string::<DefaultMessageFactory>(&message, true, Some(&dd), Some(&dd), None, false)
    //     })
    // });
}
criterion_group!{
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark_data_dictionary, criterion_benchmark_message
}
criterion_main!(benches);
