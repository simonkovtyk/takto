use std::{collections::HashMap, future::pending};

use zbus::{message, names::{BusName, InterfaceName, MemberName}, object_server::{DispatchResult, Interface, InterfaceRef}, Connection, Message, ObjectServer};

#[derive(Default)]
pub struct Tray {
  items: Vec<String>
}

impl Tray {
  async fn register_status_notifier_item (&mut self, object_server: ObjectServer, connection: Connection, message: zbus::Message) -> Result<(), zbus::Error> {
    let header = message.header();

    let object_path = message.body().deserialize::<String>().unwrap();

    println!("{:?}", header.clone());

    println!("{}", object_path);


    let reponse = connection.call_method(
      Some(header.sender().unwrap().clone()),
      object_path.clone(),
      Some("org.freedesktop.DBus.Properties"),
      "Get",
      &("org.kde.StatusNotifierItem", "IconName")
    ).await.unwrap();

    println!("Icon Name is {:?}", String::from_utf8(reponse.body().data().to_vec()));

    self.items.push(object_path);

    Ok(())
  }

  async fn protocol_version (&self) -> Result<(), zbus::Error> {
    Ok(())
  }

  async fn registered_status_notifier_items(&self, connection: Connection, message: zbus::Message) -> Result<(), zbus::Error> {
    let message = zbus::Message::

    connection.reply(call, body)
  }
}

impl Interface for Tray {
  fn name () -> InterfaceName<'static> {
    InterfaceName::from_static_str("org.kde.StatusNotifierWatcher").unwrap()
  }

  fn call (&self, object_server: &ObjectServer, connection: &Connection, message: &zbus::Message, member_name: MemberName) -> DispatchResult {
    match member_name.as_str() {
      "RegisterStatusNotifierItem" => {
        DispatchResult::Async(
          Box::pin(
            self.register_status_notifier_item(object_server.clone(), connection.clone(), message.clone())
          )
        )
      },
      "ProtocolVersion" => {
        DispatchResult::Async(
          Box::pin(
            self.protocol_version()
          )
        )
      },
      _ => {
        DispatchResult::NotFound
      }
    }
  }

  fn call_mut<'call>(&'call mut self,server: &'call ObjectServer,connection: &'call Connection,msg: &'call Message,name:MemberName<'call> ,) -> DispatchResult<'call> {
    self.call(server, connection, msg, name)
  }

  fn get_all<'life0,'life1,'life2,'life3,'life4,'life5,'life6,'async_trait>(&'life0 self,object_server: &'life1 ObjectServer,connection: &'life2 Connection,header:Option< &'life3 zbus::message::Header<'life4> > ,emitter: &'life5 zbus::object_server::SignalEmitter<'life6> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = zbus::fdo::Result<std::collections::HashMap<String,zbus::zvariant::OwnedValue> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,'life3:'async_trait,'life4:'async_trait,'life5:'async_trait,'life6:'async_trait,Self:'async_trait {
    Box::pin(async {
      Ok(HashMap::new())
    })
  }

  fn get<'life0,'life1,'life2,'life3,'life4,'life5,'life6,'life7,'async_trait>(&'life0 self,property_name: &'life1 str,server: &'life2 ObjectServer,connection: &'life3 Connection,header:Option< &'life4 zbus::message::Header<'life5> > ,emitter: &'life6 zbus::object_server::SignalEmitter<'life7> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Option<zbus::fdo::Result<zbus::zvariant::OwnedValue> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,'life3:'async_trait,'life4:'async_trait,'life5:'async_trait,'life6:'async_trait,'life7:'async_trait,Self:'async_trait {
    Box::pin(async {
      Some(zbus::fdo::Result::Ok(zbus::zvariant::OwnedValue::from(0)))
    })
  }

  fn set<'call>(&'call self,property_name: &'call str,value: &'call zbus::zvariant::Value<'_> ,object_server: &'call ObjectServer,connection: &'call Connection,header:Option< &'call zbus::message::Header<'_> > ,emitter: &'call zbus::object_server::SignalEmitter<'_> ,) -> DispatchResult<'call> {
    DispatchResult::NotFound
  }

  fn set_mut<'life0,'life1,'life2,'life3,'life4,'life5,'life6,'life7,'life8,'life9,'async_trait>(&'life0 mut self,property_name: &'life1 str,value: &'life2 zbus::zvariant::Value<'life3> ,object_server: &'life4 ObjectServer,connection: &'life5 Connection,header:Option< &'life6 zbus::message::Header<'life7> > ,emitter: &'life8 zbus::object_server::SignalEmitter<'life9> ,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Option<zbus::fdo::Result<()> > > + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,'life3:'async_trait,'life4:'async_trait,'life5:'async_trait,'life6:'async_trait,'life7:'async_trait,'life8:'async_trait,'life9:'async_trait,Self:'async_trait {
    Box::pin(async {
      Some(zbus::fdo::Result::Ok(()))
    })
  }

  fn introspect_to_writer(&self,writer: &mut dyn std::fmt::Write,level:usize) {}

  fn spawn_tasks_for_methods(&self) -> bool {
    false
  }
}

pub fn listen() -> () {
  tokio::task::spawn(async move {
    let tray = Tray::default();
    let _conn = zbus::connection::Builder::session().unwrap()
      .name("org.kde.StatusNotifierWatcher").unwrap()
      .serve_at("/StatusNotifierWatcher",  tray).unwrap()
      .build()
      .await.unwrap();

      pending::<()>().await;
  });
}
