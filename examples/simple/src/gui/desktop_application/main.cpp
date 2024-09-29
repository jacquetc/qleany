#include "entities_registration.h"
#include "controller_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);

    new Simple::Entities::EntitiesRegistration(&a);
    auto *persistenceRegistration = new Simple::Persistence::PersistenceRegistration(&a);
    new Simple::Controller::ControllerRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow w;
    w.show();
    w.init();

    return a.exec();
}
