#include "controller_registration.h"
#include "domain_registration.h"
#include "mainwindow.h"
#include "persistence_registration.h"
#include <QApplication>
#include <QCoroTask>

int main(int argc, char *argv[])
{

    QApplication a(argc, argv);

    new Simple::Domain::DomainRegistration(&a);
    auto *persistenceRegistration = new Simple::Persistence::PersistenceRegistration(&a);
    new Simple::Controller::ControllerRegistration(&a, persistenceRegistration->repositoryProvider());

    MainWindow w;
    w.show();
    w.init();

    return a.exec();
}
