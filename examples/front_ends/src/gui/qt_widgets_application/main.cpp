#include "entities_registration.h"
#include "interactor_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);
    a.setApplicationName("FrontEndsExample");
    a.setOrganizationName("frontendsexample");
    a.setOrganizationDomain("qleany.eu");

    new FrontEnds::Entities::EntitiesRegistration(&a);
    auto *persistenceRegistration = new FrontEnds::Persistence::PersistenceRegistration(&a);
    new FrontEnds::Interactor::InteractorRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow window;
    window.show();

    return a.exec();
}