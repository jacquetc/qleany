// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#pragma once

#include "front_ends_example_controller_export.h"
#include "repository/interface_repository_provider.h"
#include <QObject>

namespace FrontEnds::Controller
{

class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT ControllerRegistration : public QObject
{
    Q_OBJECT
public:
    explicit ControllerRegistration(QObject *parent, FrontEnds::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~ControllerRegistration();

private:
};

} // namespace FrontEnds::Controller