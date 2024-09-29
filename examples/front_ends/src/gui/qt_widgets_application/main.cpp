// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "entities_registration.h"
#include "controller_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);
    a.setApplicationName("FrontEndsExample"_L1);
    a.setOrganizationName("frontendsexample"_L1);
    a.setOrganizationDomain("qleany.eu"_L1);

    new FrontEnds::Entities::EntitiesRegistration(&a);
    auto *persistenceRegistration = new FrontEnds::Persistence::PersistenceRegistration(&a);
    new FrontEnds::Controller::ControllerRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow window;
    window.show();

    return a.exec();
}
